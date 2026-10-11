//! Ｒ．Ｂ．ＧＡ１０ドリラー
//! R.B. Ga10 Driller
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 6043161;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RB], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control no face-up monsters, on the only face-up monsters you control are "R.B." monsters, you can Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy both 1 monster your opponent controls and this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and (#g==0 or g:FilterCount(Card.IsSetCard,nil,SET_RB)==#g)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && (g.is_empty() || group::filter_count(d, &g, &|d, c| card::is_set_card(d, c, &[SET_RB as u16]), NO_EX) == g.len())
}
/// `s.desconfilter(c,ec,lg)`
fn desconfilter(d: &mut Duel, c: CardId, ec: CardId, lg: &CardSet) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsLinkMonster() and c:IsFaceup() and (lg:IsContains(c) or c:GetLinkedGroup():IsContains(ec))
    card::is_set_card(d, c, &[SET_RB as u16])
        && card::is_link_monster(d, c)
        && card::is_faceup(d, c)
        && (lg.contains(c) || card::get_linked_group(d, c).contains(ec))
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsMainPhase() and Duel.IsExistingMatchingCard(s.desconfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c,c:GetLinkedGroup())
    if !duel::is_main_phase(d, None) {
        return false;
    }
    let lg = card::get_linked_group(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| desconfilter(d, x, c, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,1,nil,tp)+e:GetHandler()
    let mut g = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let c = effect::get_handler(d, a.e).nil();
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsMonster() then
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_monster(d, tc) {
            // Duel.Destroy(Group.FromCards(tc,c),REASON_EFFECT)
            let mut g = CardSet::from_card(tc);
            group::add_card(&mut g, &CardSet::from_card(c));
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
