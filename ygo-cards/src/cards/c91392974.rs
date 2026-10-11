//! Apparition
use super::prelude::*;

pub const ID: u32 = 91392974;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HERO, SET_VISION_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Level 4 or lower "Vision HERO" monster from your Deck, then you can halve the original ATK/DEF of 1 monster your opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.confilter(c,tp)
fn confilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousSetCard(SET_HERO) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_previous_set_card(d, c, &[SET_HERO as u16])
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.confilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| confilter(d, c, tp), 1, NO_EX)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_VISION_HERO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_set_card(d, c, &[SET_VISION_HERO as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        let e = a.e;
        let tp = a.tp;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty()
        && duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // local sc=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sg) else {
            return;
        };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(sc:GetBaseAttack()/2)
        let atk = lua_int(card::get_base_attack(d, sc) as f64 / 2.0);
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
        effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
        // e2:SetValue(sc:GetBaseDefense()/2)
        let def = lua_int(card::get_base_defense(d, sc) as f64 / 2.0);
        effect::set_value(d, e2, def);
        // sc:RegisterEffect(e2)
        card::register_effect(d, sc, e2, false);
    }
}
