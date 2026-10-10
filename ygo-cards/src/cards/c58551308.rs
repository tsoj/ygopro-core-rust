//! Spear Cretin
use super::prelude::*;

pub const ID: u32 = 58551308;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip effect
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_FLIP);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e1, Some(operation(flipop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.flipop(e,tp,eg,ep,ev,re,r,rp)`
fn flipop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_MZONE | LOCATION_GRAVE) {
        // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD|RESET_OVERLAY)&~(RESET_LEAVE|RESET_TOGRAVE),0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | ((RESETS_STANDARD | RESET_OVERLAY) & !(RESET_LEAVE | RESET_TOGRAVE)), 0, 1, 0, 0);
    }
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)~=0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return card::get_flag_effect(d, h, ID) != 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler(),e,tp)
    let h = effect::get_handler(d, e);
    let mut g1 = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, h.map_or(NO_EX, |h| h.into())).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectTarget(1-tp,s.filter,1-tp,LOCATION_GRAVE,0,1,1,e:GetHandler(),e,1-tp)
    let g2 = duel::select_target(d, 1 - tp, Some(&|d, c| filter(d, c, e, 1 - tp)), 1 - tp, LOCATION_GRAVE, 0, 1, 1, false, h.map_or(NO_EX, |h| h.into())).nil();
    // if #g1>0 and #g2>0 then
    if !g1.is_empty() && !g2.is_empty() {
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g1,2,PLAYER_ALL,g1:GetFirst():GetOwner())
        let first = group::get_first(d, &g1).nil();
        let owner = card::get_owner(d, first);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g1), 2, PLAYER_ALL as u8, owner as i32);
    } else if !g1.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g1,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g1), 1, 0, 0);
    } else if !g2.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g2,1,1,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g2), 1, 1, 0);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetTargetCards(e)
    let sg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // if Duel.GetLocationCount(tc:GetControler(),LOCATION_MZONE)>0 then
        let ctrl = card::get_controler(d, tc);
        if duel::get_location_count(d, ctrl, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            // local sp=tc:GetControler()
            let sp = card::get_controler(d, tc);
            // Duel.SpecialSummonStep(tc,0,sp,sp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
            duel::special_summon_step(d, tc, 0, sp, sp, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
