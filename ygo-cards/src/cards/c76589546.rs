//! Overflow Dragon
use super::prelude::*;

pub const ID: u32 = 76589546;
const TOKEN_OVERFLOW: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_OVERFLOW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When a monster(s) on the field is destroyed by card effect (except during the Damage Step): You can Special Summon this card from your hand, then, if 2 or more monsters on the field were destroyed by that card effect, you can Special Summon 1 "Overflow Token" (Dragon/DARK/Level 1/ATK 0/DEF 0). You can only use this effect of "Overflow Dragon" once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
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
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsReason(REASON_EFFECT)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &spconfilter, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // if eg:IsExists(s.spconfilter,2,nil) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &spconfilter, 2, NO_EX) {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,2,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 2, a.tp, 0);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetLabel()==1
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_OVERFLOW,0,TYPES_TOKEN,0,0,1,RACE_DRAGON,ATTRIBUTE_DARK)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && effect::get_label(d, a.e) == 1
        && duel::is_player_can_special_summon_monster(
            d,
            tp,
            TOKEN_OVERFLOW,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPES_TOKEN),
                attack: Some(0),
                defense: Some(0),
                level: Some(1),
                race: Some(RACE_DRAGON),
                attribute: Some(ATTRIBUTE_DARK),
            },
            None,
            None,
            0,
        )
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1) as u64)
    {
        // local token=Duel.CreateToken(tp,TOKEN_OVERFLOW)
        let token = duel::create_token(d, tp, TOKEN_OVERFLOW);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
