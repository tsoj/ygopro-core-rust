//! Skelesaurus
use super::prelude::*;

pub const ID: u32 = 89362180;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gemini.AddProcedure(c)
    gemini::add_procedure(d, c);
    // --Becomes EARTH Dinosaur monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(Gemini.EffectStatusCondition)
    effect::set_condition(d, e1, Some(gemini::effect_status_condition()));
    // e1:SetValue(ATTRIBUTE_EARTH)
    effect::set_value(d, e1, ATTRIBUTE_EARTH as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e2, EFFECT_CHANGE_RACE);
    // e2:SetValue(RACE_DINOSAUR)
    effect::set_value(d, e2, RACE_DINOSAUR as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Special Summon destroyed monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Gemini.EffectStatusCondition(e) then return false end
    if !gemini::effect_status_condition().call(d, &[Value::Effect(a.e)]).truthy() {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToBattle() or c:IsFacedown() then return false end
    if !card::is_relate_to_battle(d, c) || card::is_facedown(d, c) {
        return false;
    }
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster()
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc),
        None => false,
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and bc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, bc.nil(), a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, bc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(bc.nil())), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0 {
        // --Treated as a Zombie monster
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetValue(RACE_ZOMBIE)
        effect::set_value(d, e1, RACE_ZOMBIE as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
