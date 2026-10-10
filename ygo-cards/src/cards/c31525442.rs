//! Zolga the Prophet
//! Scripted by The Razgriz
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 31525442;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon/Look at top 5 cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(cost_target(tg)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FAIRY) and c:IsAttribute(ATTRIBUTE_EARTH) and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FAIRY) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && !card::is_code(d, c, &[ID])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 or Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None)
            && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0
            || duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // s.look(tp,tp)
        look(d, a.tp, a.tp);
        // s.look(tp,1-tp)
        look(d, a.tp, 1 - a.tp);
    }
}
/// `s.look(tp,p)`
fn look(d: &mut Duel, tp: u8, p: u8) {
    // local gc=math.min(5,Duel.GetFieldGroupCount(p,LOCATION_DECK,0))
    let gc = std::cmp::min(5, duel::get_field_group_count(d, p, LOCATION_DECK, 0));
    // if gc>0 then
    if gc > 0 {
        // local ac=gc==1 and gc or Duel.AnnounceNumberRange(tp,1,gc)
        let ac = if gc == 1 {
            gc
        } else {
            duel::announce_number_range(d, tp, Some(1), Some(gc as u32), &[]).nil().0 as i32
        };
        // Duel.ConfirmCards(tp,Duel.GetDecktopGroup(p,ac))
        let g = duel::get_decktop_group(d, p, ac as u32);
        duel::confirm_cards(d, tp, g);
    }
}
/// `s.con(e,tp,eg,ev,ep,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetReasonCard()
    let tc = card::get_reason_card(d, c);
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // return (e:GetHandler():IsReason(REASON_RELEASE) and not e:GetHandler():IsReason(REASON_EFFECT))
    //     and tc and tc:IsMonster() and at==tc
    (card::is_reason(d, c, REASON_RELEASE) && !card::is_reason(d, c, REASON_EFFECT))
        && tc.is_some()
        && card::is_monster(d, tc.nil())
        && at == tc
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local tc=e:GetHandler():GetReasonCard()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_reason_card(d, c);
    // if (e:GetHandler():IsReason(REASON_RELEASE) and not e:GetHandler():IsReason(REASON_EFFECT)) and tc:IsReason(REASON_SUMMON)
    //     and tc:IsMonster() and at==tc then
    if (card::is_reason(d, c, REASON_RELEASE) && !card::is_reason(d, c, REASON_EFFECT))
        && card::is_reason(d, tc.nil(), REASON_SUMMON)
        && card::is_monster(d, tc.nil())
        && at == tc
    {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc.nil())), 1, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,2000)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 2000);
    }
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetHandler():GetReasonCard()
    let tc = card::get_reason_card(d, c).nil();
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if not tc:IsReason(REASON_SUMMON) or (not c:IsReason(REASON_RELEASE) or c:IsReason(REASON_EFFECT)) and not at==tc then return end
    // (Lua: "not at==tc" is "(not at)==tc", a boolean compared with a card, so it is always false)
    let not_at_eq_tc = false;
    if !card::is_reason(d, tc, REASON_SUMMON)
        || ((!card::is_reason(d, c, REASON_RELEASE) || card::is_reason(d, c, REASON_EFFECT)) && not_at_eq_tc)
    {
        return;
    }
    // if at==tc then
    if at == Some(tc) {
        // if Duel.Destroy(tc,REASON_EFFECT)~=0 then
        if duel::destroy(d, tc, REASON_EFFECT, None, None) != 0 {
            // Duel.Damage(1-tp,2000,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, 2000, REASON_EFFECT, false, None);
        }
    }
}
