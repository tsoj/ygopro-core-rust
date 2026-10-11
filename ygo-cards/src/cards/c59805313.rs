//! Volcanic Rimfire
use super::prelude::*;

pub const ID: u32 = 59805313;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_VOLCANIC, SET_BLAZE_ACCELERATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VOLCANIC) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_VOLCANIC as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
/// `s.rmfilter(c,tp,ft)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8, ft: i32) -> bool {
    // return (ft>0 or c:IsLocation(LOCATION_STZONE)) and c:IsSetCard(SET_BLAZE_ACCELERATOR) and c:IsAbleToRemove() and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,tp)
    (ft > 0 || card::is_location(d, c, LOCATION_STZONE))
        && card::is_set_card(d, c, &[SET_BLAZE_ACCELERATOR as u16])
        && card::is_able_to_remove(d, c, None, None, None)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BLAZE_ACCELERATOR) and c:IsContinuousSpellTrap() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_BLAZE_ACCELERATOR as u16])
        && card::is_continuous_spell_trap(d, c)
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // local b1=not Duel.HasFlagEffect(tp,id) and c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None)
        && card::is_able_to_remove(d, c, None, None, None)
        && duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil,tp,ft)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| rmfilter(d, x, a.tp, ft)), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    let op_num = op.map_or(0, |x| x as i64);
    effect::set_label(d, a.e, &[op_num]);
    // if op==1 then
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_REMOVE + CATEGORY_TOGRAVE);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_GRAVE as i32);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_REMOVE)
        effect::set_category(d, a.e, CATEGORY_REMOVE);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_ONFIELD|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not c:IsRelateToEffect(e) or Duel.Remove(c,POS_FACEUP,REASON_EFFECT)==0 then return end
        if !card::is_relate_to_effect(d, c, a.e) || duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,1,nil,tp,ft)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| rmfilter(d, x, a.tp, ft)), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 or Duel.Remove(g,POS_FACEUP,REASON_EFFECT)==0 then return end
        if g.is_empty() || duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local plc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,tp):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| plfilter(d, x, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let plc = group::get_first(d, &sel).nil();
        // Duel.MoveToField(plc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, plc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
