//! Volcanic Emperor
//! ヴォルカニック・エミッション
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 46412900;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_BLAZE_ACCELERATOR, SET_VOLCANIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand or GY) by banishing 3 Pyro monsters or 1 "Blaze Accelerator" card from your face-up field and/or GY
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e0, LOCATION_HAND | LOCATION_GRAVE);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(|d, a| spcon(d, a))));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(lua_fn(|d, a| sptg(d, a))));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(lua_fn(|d, a| {
        spop(d, a);
        Value::Nil
    })));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Inflict 500 damage to your opponent for each banished Pyro monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) end)
    effect::set_condition(d, e1, Some(lua_fn(|d, a| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
    })));
    // e1:SetTarget(s.damsettg)
    effect::set_target(d, e1, Some(cost_target(damsettg)));
    // e1:SetOperation(s.damsetop)
    effect::set_operation(d, e1, Some(operation(damsetop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Each time your opponent Special Summons a monster(s), inflict 500 damage to your opponent
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcostfilter(c,tp)
fn spcostfilter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return ((c:IsMonster() and c:IsRace(RACE_PYRO)) or c:IsSetCard(SET_BLAZE_ACCELERATOR))
    //     and c:IsFaceup() and c:IsAbleToRemoveAsCost()
    ((card::is_monster(d, c) && card::is_race(d, c, RACE_PYRO)) || card::is_set_card(d, c, &[SET_BLAZE_ACCELERATOR as u16]))
        && card::is_faceup(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0
    //     and ((sg:IsExists(Card.IsSetCard,1,nil,SET_BLAZE_ACCELERATOR) and #sg==1)
    //     or sg:FilterCount(Card.IsRace,nil,RACE_PYRO)==3)
    let mz = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0;
    let res = mz
        && ((group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_BLAZE_ACCELERATOR as u16]), 1, NO_EX) && sg.len() == 1)
            || group::filter_count(d, sg, &|d, c| card::is_race(d, c, RACE_PYRO), NO_EX) == 3);
    (res, false)
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args<'_>) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let e = a.e(0);
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,c)
    let rg = duel::get_matching_group(d, Some(&|d, x| spcostfilter(d, x, tp)), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, c.into());
    // return #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,3,s.rescon,0)
    !rg.is_empty()
        && matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(3), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn sptg(d: &mut Duel, a: Args<'_>) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=nil
    // local rg=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,c)
    let c = a.card(9);
    let ex: Exclude = c.map_or(NO_EX, |c| c.into());
    let rg = duel::get_matching_group(d, Some(&|d, x| spcostfilter(d, x, tp)), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, ex);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,3,s.rescon,1,tp,HINTMSG_REMOVE,s.rescon,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), Some(&rescon), None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args<'_>) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
// function s.damsettg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damsettg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_PYRO),tp,LOCATION_REMOVED,LOCATION_REMOVED,nil)
    let f = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_PYRO));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (g.len() * 500) as i32);
    true
}
// function s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VOLCANIC) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_VOLCANIC as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// function s.damsetop(e,tp,eg,ep,ev,re,r,rp)
fn damsetop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_PYRO),tp,LOCATION_REMOVED,LOCATION_REMOVED,nil)
    let f = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_PYRO));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if Duel.Damage(1-tp,#g*500,REASON_EFFECT)>0
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::damage(d, 1 - tp, (g.len() * 500) as i64, REASON_EFFECT, false, None) > 0
        && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
        let sg = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
        // if #sg>0 then
        if let Some(sg) = sg.filter(|sg| !sg.is_empty()) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SSet(tp,sg)
            duel::s_set(d, tp, &sg, None, true);
        }
    }
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Damage(1-tp,500,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, 500, REASON_EFFECT, false, None);
}
