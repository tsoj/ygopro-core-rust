//! Triamid Hunter
use super::prelude::*;

pub const ID: u32 = 95923441;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRIAMID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Additional Norma Summon of a Rock Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(condition(sumcon)));
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_ROCK))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_ROCK))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Replace a Field Spell
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(turncond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end` (the second effect's condition)
fn turncond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.sumcon(e)`
fn sumcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `s.filter(c,tp,code)`
fn filter(d: &mut Duel, c: CardId, tp: u8, code: u32) -> bool {
    // return c:IsFieldSpell() and c:IsSetCard(SET_TRIAMID) and c:GetActivateEffect():IsActivatable(tp,true,true) and not c:IsCode(code)
    let act = card::get_activate_effect(d, c).first().copied().nil();
    card::is_field_spell(d, c) && card::is_set_card(d, c, &[SET_TRIAMID as u16]) && effect::is_activatable(d, act, tp, true, true) && !card::is_code(d, c, &[code])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let tc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return tc and tc:IsFaceup() and tc:IsSetCard(SET_TRIAMID) and tc:IsAbleToGrave() and tc:IsCanBeEffectTarget(e)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,tp,tc:GetCode()) end
    if a.chk == Some(0) {
        return match tc {
            None => false,
            Some(tc) => {
                card::is_faceup(d, tc)
                    && card::is_set_card(d, tc, &[SET_TRIAMID as u16])
                    && card::is_able_to_grave(d, tc, None)
                    && card::is_can_be_effect_target(d, tc, Some(a.e))
                    && {
                        let code = card::get_code(d, tc)[0];
                        duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, a.tp, code)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
                    }
            }
        };
    }
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(tc.nil())), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoGrave(tc,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_GRAVE) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_GRAVE) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local fc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,tp,tc:GetCode()):GetFirst()
        let code = card::get_code(d, tc)[0];
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, a.tp, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let fc = group::get_first(d, &sel);
        // Duel.ActivateFieldSpell(fc,e,tp,eg,ep,ev,re,r,rp)
        duel::activate_field_spell(d, fc, a, None);
    }
}
