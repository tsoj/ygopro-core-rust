//! Chanbar, the Flashy Sportsknight
//! 明滅騎士チャンバル
use super::prelude::*;

pub const ID: u32 = 50643638;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Monsters your opponent controls lose 400 ATK for each Equip Spell you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 1 Equip Spell from your hand or Deck to the GY, and if you do, destroy that target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tgdestg)
    effect::set_target(d, e2, Some(cost_target(tgdestg)));
    // e2:SetOperation(s.tgdesop)
    effect::set_operation(d, e2, Some(operation(tgdesop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 2 face-up cards in your Spell & Trap Zone, and if you do, Special Summon this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkdvalfilter(c)`
fn atkdvalfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsEquipSpell() and (c:IsFaceup() or c:GetEquipTarget())
    card::is_equip_spell(d, c) && (card::is_faceup(d, c) || card::get_equip_target(d, c).is_some())
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return -400*Duel.GetMatchingGroupCount(s.atkdvalfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,nil)
    let p = effect::get_handler_player(d, e);
    -400 * duel::get_matching_group_count(d, Some(&atkdvalfilter), p, LOCATION_ONFIELD, 0, NO_EX) as i64
}
/// `s.tgdestg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsOnField() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsEquipSpell,Card.IsAbleToGrave),tp,LOCATION_HAND|LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_equip_spell(d, x) && card::is_able_to_grave(d, x, None)),
            tp,
            LOCATION_HAND | LOCATION_DECK,
            0,
            1,
            NO_EX,
        ) && duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    true
}
/// `s.tgdesop(e,tp,eg,ep,ev,re,r,rp)`
fn tgdesop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.AND(Card.IsEquipSpell,Card.IsAbleToGrave),tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| card::is_equip_spell(d, x) && card::is_able_to_grave(d, x, None)),
        tp,
        LOCATION_HAND | LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE) and tc:IsRelateToEffect(e) then
    if let Some(sc) = sc {
        if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) > 0 && card::is_location(d, sc, LOCATION_GRAVE) {
            let tc = tc.nil();
            if card::is_relate_to_effect(d, tc, e) {
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, tc, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_STZONE,0,2,nil)
    //     and Duel.GetMZoneCount(tp)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_STZONE, 0, 2, NO_EX)
            && duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_STZONE,0,2,2,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_STZONE, 0, 2, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // if #tg>0 and Duel.Destroy(tg,REASON_EFFECT)>0 and c:IsRelateToEffect(e) then
    if !tg.is_empty()
        && duel::destroy(d, tg, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
