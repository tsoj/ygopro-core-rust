//! Rite of Aramesir
use super::prelude::*;

pub const ID: u32 = 3285551;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ADVENTURER, 39568067], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Adventurer Token"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Count activated monster effects
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.actfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(actfilter));
}
/// The body of `s.actfilter(re)`
fn actfilter_impl(d: &mut Duel, re: EffectId) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return not (re:IsMonsterEffect() and rc:IsOnField() and not rc:IsSpecialSummoned())
    !(effect::is_monster_effect(d, re) && card::is_on_field(d, rc) && !card::is_special_summoned(d, rc))
}
/// `s.actfilter(re)` as a counter function
fn actfilter(d: &mut Duel, a: Args) -> bool {
    actfilter_impl(d, a.e(0))
}
/// `s.spcon(e,tp,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,TOKEN_ADVENTURER),tp,LOCATION_ONFIELD,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[TOKEN_ADVENTURER])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `function(_,re) return not s.actfilter(re) end` (the value of the lock effect)
fn lock_value(d: &mut Duel, a: Args) -> bool {
    !actfilter_impl(d, a.e(1))
}
/// `s.spcost(e,tp,...,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_CHAIN) == 0;
    }
    // Cannot activate effects of monsters, except Special Summoned monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(_,re) return not s.actfilter(re) end)
    effect::set_value_fn(d, e1, lua_fn(lock_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// The body of `s.sptg` when called with chk == 0 (`Duel.IsPlayerCanSpecialSummonMonster` for the token)
fn can_sptg(d: &mut Duel, tp: u8) -> bool {
    // Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    // and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ADVENTURER,0,TYPES_TOKEN,2000,2000,4,RACE_FAIRY,ATTRIBUTE_EARTH)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(
            d,
            tp,
            TOKEN_ADVENTURER,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPES_TOKEN),
                attack: Some(2000),
                defense: Some(2000),
                level: Some(4),
                race: Some(RACE_FAIRY),
                attribute: Some(ATTRIBUTE_EARTH),
            },
            None,
            None,
            0,
        )
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return ... end
    if a.chk == Some(0) {
        return can_sptg(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(39568067) and not c:IsForbidden()
    card::is_code(d, c, &[39568067]) && !card::is_forbidden(d, c)
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if not s.sptg(e,tp,eg,ep,ev,re,r,rp,0) then return end
    if !can_sptg(d, a.tp) {
        return;
    }
    // local token=Duel.CreateToken(tp,TOKEN_ADVENTURER)
    let token = duel::create_token(d, a.tp, TOKEN_ADVENTURER).nil();
    // if Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,39568067),tp,LOCATION_ONFIELD,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[39568067])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&plfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if tc then
        if let Some(tc) = group::get_first(d, &g) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        }
    }
}
