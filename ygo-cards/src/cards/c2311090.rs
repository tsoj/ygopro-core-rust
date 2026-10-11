//! Spright Carrot
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 2311090;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Special Summon self
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Negate Spell/Trap effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetCost(s.discost)
    effect::set_cost(d, e2, Some(cost_target(discost)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsLevel(2) or c:IsLink(2))
    card::is_faceup(d, c) && (card::is_level(d, c, &[2]) || card::is_link(d, c, &[2]))
}
/// `s.spcon(e,c)`: `function(e,c)`, the core checks the effect alone with no card
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(_c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsSpellTrapEffect() and Duel.IsChainDisablable(ev)
    a.rp == 1 - a.tp && effect::is_spell_trap_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.discostfilter(c)`
fn discostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(2) or c:IsRank(2) or c:IsLink(2)
    card::is_level(d, c, &[2]) || card::is_rank(d, c, &[2]) || card::is_link(d, c, &[2])
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.discostfilter,1,false,nil,c) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&discostfilter), 1, 1, false, None, c.into());
    }
    // local rc=Duel.SelectReleaseGroupCost(tp,s.discostfilter,1,1,false,nil,c):GetFirst()
    let g = duel::select_release_group_cost(d, a.tp, Some(&discostfilter), 1, 1, false, None, c.into());
    let rc = group::get_first(d, &g).nil();
    // e:SetLabel(rc:GetLevel())
    let level = card::get_level(d, rc) as i64;
    effect::set_label(d, a.e, &[level]);
    // Duel.Release(rc,REASON_COST)
    duel::release(d, rc, REASON_COST, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not re:GetHandler():IsDisabled() end
    if a.chk == Some(0) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        return !card::is_disabled(d, rc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re)
    //   and e:GetLabel()==0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re)
            && effect::get_label(d, a.e) == 0
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
