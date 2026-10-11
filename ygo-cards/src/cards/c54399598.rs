//! Fleeting Flower of the Magician
use super::prelude::*;

pub const ID: u32 = 54399598;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 Spell/Trap on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_LEAVE_GRAVE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_LEAVE_GRAVE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and (c:IsFaceup() or c:IsControler(tp))
    card::is_spell_trap(d, c) && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // if chkc then return chkc~=c and chkc:IsOnField() and s.desfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return chkc != c && card::is_on_field(d, chkc) && desfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, tp)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c,tp):GetFirst()
    let g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, tp)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
    let tc = group::get_first(d, &g).nil();
    // e:SetLabel(tc:IsControler(tp) and 106 or 0)
    let label = if card::is_controler(d, tc, tp) { 106 } else { 0 };
    effect::set_label(d, a.e, &[label]);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_monster(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsEquipSpell() or c:IsContinuousSpell()) and c:IsAbleToHand()
    (card::is_equip_spell(d, c) || card::is_continuous_spell(d, c)) && card::is_able_to_hand(d, c, None)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or Duel.Destroy(tc,REASON_EFFECT)==0 or e:GetLabel()~=106 then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    if duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    if effect::get_label(d, a.e) != 106 {
        return;
    }
    // local plg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.plfilter),tp,LOCATION_GRAVE,0,nil,tp)
    let plg_f = aux::necro_valley_filter(move |d, c| plfilter(d, c, tp));
    let plg = duel::get_matching_group(d, Some(&plg_f), tp, LOCATION_GRAVE, 0, NO_EX);
    // local thg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,nil)
    let thg_f = aux::necro_valley_filter(|d, c| thfilter(d, c));
    let thg = duel::get_matching_group(d, Some(&thg_f), tp, LOCATION_GRAVE, 0, NO_EX);
    // local b1=#plg>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let b1 = !plg.is_empty() && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    // local b2=#thg>0
    let b2 = !thg.is_empty();
    // if not (b1 or b2) or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if !(b1 || b2) || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local plc=plg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &plg, tp, 1, 1, false, NO_EX).nil();
        let plc = group::get_first(d, &sel);
        // if not plc then return end
        let Some(plc) = plc else { return };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.MoveToField(plc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
        if duel::move_to_field(d, plc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // plc:RegisterEffect(e1)
            card::register_effect(d, plc, e1, false);
        }
    } else if op == Some(2) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local hg=thg:Select(tp,1,1,nil)
        let hg = group::select(d, &thg, tp, 1, 1, false, NO_EX).nil();
        // if #hg>0 then
        if !hg.is_empty() {
            // Duel.HintSelection(hg)
            duel::hint_selection(d, hg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(hg,nil,REASON_EFFECT)
            duel::send_to_hand(d, hg, None, REASON_EFFECT, None);
        }
    }
}
