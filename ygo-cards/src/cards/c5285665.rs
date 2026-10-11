//! Elemental HERO Neo Bubbleman
use super::prelude::*;

pub const ID: u32 = 5285665;

// s.listed_names={79979666,46411259} --Elemental HERO Bubbleman, Metamorphosis
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[79979666, 46411259], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be Special Summoned, except by its own effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // This card's name becomes "Elemental HERO Bubbleman"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e3, EFFECT_CHANGE_CODE);
    // e3:SetValue(79979666)
    effect::set_value(d, e3, 79979666);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy a monster that battles with this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spfilter(c,...)`: `c:IsCode(...) and c:IsAbleToGraveAsCost()`
fn spfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_code(d, c, &[code]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg)
    let (ok, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, None);
    let res = ok && group::is_exists(d, sg, &|d, x| chk(d, x, sg), 1, NO_EX);
    (res, false)
}
/// `s.chk(c,sg)`
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsCode(79979666) and sg:IsExists(Card.IsCode,1,c,46411259)
    card::is_code(d, c, &[79979666]) && group::is_exists(d, sg, &|d, x| card::is_code(d, x, &[46411259]), 1, c.into())
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,79979666)
    let g1 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, 79979666)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,46411259)
    let g2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, 46411259)), tp, LOCATION_HAND, 0, NO_EX);
    // local g=g1:Clone()
    // g:Merge(g2)
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-1 and #g1>0 and #g2>0
    //   and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    let _ = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
    if g1.is_empty() || g2.is_empty() {
        return false;
    }
    let rs: aux::Rescon = &rescon;
    matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,79979666)
    let mut sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, 79979666)), tp, LOCATION_MZONE, 0, NO_EX);
    // sg:Merge(Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,46411259))
    let g2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, 46411259)), tp, LOCATION_HAND, 0, NO_EX);
    group::add_card(&mut sg, &g2);
    // local g=aux.SelectUnselectGroup(sg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let g = aux::select_unselect_group(d, &sg, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
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
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
        // g:DeleteGroup()
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if chk==0 then return bc and bc:IsRelateToBattle() end
    if a.chk == Some(0) {
        return bc.is_some_and(|b| card::is_relate_to_battle(d, b));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if bc and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            // Duel.Destroy(bc,REASON_EFFECT)
            duel::destroy(d, bc, REASON_EFFECT, None, None);
        }
    }
}
