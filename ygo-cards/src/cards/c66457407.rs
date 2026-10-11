//! Copy Plant
//! コピー・プラント
use super::prelude::*;

pub const ID: u32 = 66457407;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change this card's Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lvfilter(c,lvl)`
fn lvfilter(d: &mut Duel, c: CardId, lvl: u32) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_PLANT) and c:HasLevel() and not c:IsLevel(lvl)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT) && card::has_level(d, c) && !card::is_level(d, c, &[lvl])
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lvl=c:GetLevel()
    let lvl = card::get_level(d, c);
    // if chkc then return chkc~=c and chkc:IsLocation(LOCATION_MZONE) and s.lvfilter(chkc,lvl) end
    if let Some(chkc) = a.chkc() {
        return chkc != c && card::is_location(d, chkc, LOCATION_MZONE) && lvfilter(d, chkc, lvl);
    }
    // if chk==0 then return lvl>0 and Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,lvl) end
    if a.chk == Some(0) {
        return lvl > 0
            && duel::is_existing_target(d, Some(&|d, cc| lvfilter(d, cc, lvl)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,lvl)
    duel::select_target(d, tp, Some(&|d, cc| lvfilter(d, cc, lvl)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsFaceup() and c:IsRelateToEffect(e)) then return end
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e)) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Change this card's Level
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(tc:GetLevel())
        let lv = card::get_level(d, tc);
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
