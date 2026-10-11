//! Master Peace, the True Dracoslayer
use super::prelude::*;

pub const ID: u32 = 34079868;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOSLAYER, SET_DRACOVERLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon condition
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg)
    let mmz = (aux::chkf_mmz(1))(d, sg, e, tp, mg, c).0;
    (mmz && group::is_exists(d, sg, &|d, x| chk(d, x, sg), 1, NO_EX), false)
}
// function s.chk(c,sg)
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsSetCard(SET_DRACOSLAYER) and sg:IsExists(Card.IsSetCard,1,c,SET_DRACOVERLORD)
    card::is_set_card(d, c, &[SET_DRACOSLAYER as u16]) && group::is_exists(d, sg, &|d, x| card::is_set_card(d, x, &[SET_DRACOVERLORD as u16]), 1, c.into())
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // local g1=rg:Filter(Card.IsSetCard,nil,SET_DRACOSLAYER)
    let g1 = group::filter(d, &rg, &|d, x| card::is_set_card(d, x, &[SET_DRACOSLAYER as u16]), NO_EX);
    // local g2=rg:Filter(Card.IsSetCard,nil,SET_DRACOVERLORD)
    let g2 = group::filter(d, &rg, &|d, x| card::is_set_card(d, x, &[SET_DRACOVERLORD as u16]), NO_EX);
    // local g=g1:Clone()
    // g:Merge(g2)
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #g1>0 and #g2>0 and #g>1
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    let rs: aux::Rescon = &rescon;
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && !g1.is_empty()
        && !g2.is_empty()
        && g.len() > 1
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,c)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // local g1=rg:Filter(Card.IsSetCard,nil,SET_DRACOSLAYER)
    let g1 = group::filter(d, &rg, &|d, x| card::is_set_card(d, x, &[SET_DRACOSLAYER as u16]), NO_EX);
    // local g2=rg:Filter(Card.IsSetCard,nil,SET_DRACOVERLORD)
    let g2 = group::filter(d, &rg, &|d, x| card::is_set_card(d, x, &[SET_DRACOVERLORD as u16]), NO_EX);
    // g1:Merge(g2)
    let mut g1 = g1;
    group::add_card(&mut g1, &g2);
    // local sg=aux.SelectUnselectGroup(g1,e,tp,2,2,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g1, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // sg:KeepAlive()
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD)
    a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // return Duel.IsChainNegatable(ev)
    duel::is_chain_negatable(d, a.ev as u8)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) and re:GetHandler():IsDestructable() then
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, h, re) && card::is_destructable(d, h, None) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
// function s.spfilter1(c,e,tp)
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DRACOSLAYER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_DECK,0,1,c,e,tp)
    card::is_set_card(d, c, &[SET_DRACOSLAYER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, c.into())
}
// function s.spfilter2(c,e,tp)
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DRACOVERLORD) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DRACOVERLORD as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && !duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter1(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2
    //     or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let mut g1 = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter1(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_DECK,0,1,1,g1:GetFirst(),e,tp)
    let first = group::get_first(d, &g1);
    let ex = match first {
        Some(x) => x.into(),
        None => NO_EX,
    };
    let g2 = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter2(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, ex).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // if #g1==2 then
    if g1.len() == 2 {
        // Duel.SpecialSummon(g1,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &g1, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
