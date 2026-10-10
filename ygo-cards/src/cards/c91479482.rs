//! Courageous Crimson Chevalier Bradamante
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 91479482;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNOBLE_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip 1 Equip Spell from your Deck to your Warrior monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Keep track if it was destroyed with an Equip Card or not
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e3, Some(operation(leavepop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_WARRIOR) and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK,0,1,nil,c,tp)
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_WARRIOR)
        && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.eqfilter(c,tc,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tc: CardId, tp: u8) -> bool {
    // return c:IsEquipSpell() and c:CheckEquipTarget(tc) and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    card::is_equip_spell(d, c) && card::check_equip_target(d, c, tc) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,...,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.eqop(e,tp,...)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_DECK,0,1,1,nil,tc,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| eqfilter(d, x, tc, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Equip(tp,g:GetFirst(),tc)
        let first = group::get_first(d, &g).nil();
        duel::equip(d, a.tp, first, tc, true, false);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsLocation(LOCATION_GRAVE) and rp==1-tp and bc:IsFaceup() and bc:IsRelateToBattle() and e:GetLabel()==1
    if !card::is_location(d, c, LOCATION_GRAVE) {
        return false;
    }
    if a.rp != 1 - a.tp {
        return false;
    }
    let bc = bc.nil();
    card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc) && effect::get_label(d, a.e) == 1
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and bc:IsControler(1-tp) and bc:IsAbleToChangeControler() end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && card::is_controler(d, bc.nil(), 1 - a.tp)
            && card::is_able_to_change_controler(d, bc.nil());
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,bc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, bc.map(CardSet::from), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // if not (bc:IsFaceup() and bc:IsRelateToBattle() and bc:IsControler(1-tp)) then return end
    if !(card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp)) {
        return;
    }
    // if Duel.Equip(tp,bc,c) then
    if duel::equip(d, a.tp, bc, c, true, false) {
        // Equip limit
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetLabelObject(c)
        effect::set_label_object(d, e1, Value::from(c));
        // e1:SetValue(function(e,c) return e:GetLabelObject()==c end)
        effect::set_value_fn(d, e1, effect_card_fn(equip_limit));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
}
/// `function(e,c) return e:GetLabelObject()==c end`
fn equip_limit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
/// `s.leavepop(e,tp,eg,ep,ev,re,r,rp)`
fn leavepop(d: &mut Duel, a: &EffArgs) {
    // local eff=e:GetLabelObject()
    let eff = effect::get_label_object(d, a.e).as_effect().nil();
    // if r&REASON_BATTLE>0 and #e:GetHandler():GetEquipGroup()==0 then
    let h = effect::get_handler(d, a.e).nil();
    if a.r & REASON_BATTLE > 0 && card::get_equip_group(d, h).is_empty() {
        // eff:SetLabel(1)
        effect::set_label(d, eff, &[1]);
    } else {
        // eff:SetLabel(0)
        effect::set_label(d, eff, &[0]);
    }
}
