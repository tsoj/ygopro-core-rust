//! Sennet Per Akh (--光帰への契り)
use super::prelude::*;

pub const ID: u32 = 100458007;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZENET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Add 1 "Sennet" monster from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --If you discard a "Sennet" Ritual Monster(s) while this card is in your GY: You can banish this card; Special Summon 1 non-Effect Fusion Monster from your Extra Deck with ATK less than or equal to any of the discarded monsters' as a Normal Monster Card, but for the rest of this turn, it cannot attack directly, also you cannot activate non-Zombie monster effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_DISCARD)
    effect::set_code(d, e2, EVENT_DISCARD);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Prevent the non-Effect Fusion Monster Special Summoned as a Normal Monster Card from being returned to the hand or Main Deck as cost
    // aux.GlobalCheck(s,function() ... end)
    // (the redefinition of Card.IsAbleToHandAsCost / Card.IsAbleToDeckAsCost lives in lua/card.rs, AS_COST_GUARDS)
    aux::global_check(d, ID, |_| {});
}
// function s.thfilter(c)
//     return c:IsSetCard(SET_ZENET) and c:IsMonster() and c:IsAbleToHand()
// end
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ZENET as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.spconfilter(c,tp)
//     return c:IsPreviousControler(tp) and c:IsSetCard(SET_ZENET) and c:IsRitualMonster()
// end
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_controler(d, c, tp) && card::is_set_card(d, c, &[SET_ZENET as u16]) && card::is_ritual_monster(d, c)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
//     return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.spconfilter,1,nil,tp)
// end
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| spconfilter(d, c, a.tp), 1, NO_EX)
}
// function s.spfilter(c,e,tp,max_atk)
//     return c:IsNonEffectMonster() and c:IsFusionMonster() and c:IsAttackBelow(max_atk)
//         and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
//         and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
// end
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, max_atk: Option<i64>) -> bool {
    // (a nil max_atk is 0 in the C++ lua_get<int32_t>)
    card::is_non_effect_monster(d, c)
        && card::is_fusion_monster(d, c)
        && card::is_attack_below(d, c, max_atk.unwrap_or(0) as i32)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local _,max_atk=eg:Filter(s.spconfilter,nil,tp):GetMaxGroup(Card.GetTextAttack)
    let eg = a.eg.clone().nil();
    let filtered = group::filter(d, &eg, &|d, c| spconfilter(d, c, tp), NO_EX);
    let max_atk = group::get_max_group(d, &filtered, &|d, c| card::get_text_attack(d, c) as i64).map(|(_, v)| v);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,max_atk) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, max_atk)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // e:GetChainData().max_atk=max_atk
    let v = match max_atk {
        Some(x) => Value::Int(x),
        None => Value::Nil,
    };
    chain::set_effect_data(d, a.e, None, chain::field::CARD, v);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local max_atk=e:GetChainData().max_atk
    let max_atk = match chain::effect_data(d, a.e, None, chain::field::CARD) {
        Value::Int(x) => Some(x),
        _ => None,
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,max_atk):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, max_atk)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // local original_type=sc:GetType()
            let original_type = card::get_type(d, sc);
            // sc:Type((original_type&~TYPE_FUSION)|TYPE_NORMAL)
            card::info_type(d, sc, Some((original_type & !TYPE_FUSION) | TYPE_NORMAL));
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,original_type,aux.Stringid(id,2))
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, original_type as i64, aux::stringid(ID, 2));
            // Reset its type back to the original when it stops being face-up in the Monster Zone
            restore_type_on_leave(d, c, sc, original_type, tp);
            // But for the rest of this turn, it cannot attack directly
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3207)
            effect::set_description(d, e1, 3207);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Also you cannot activate non-Zombie monster effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e2, aux::stringid(ID, 3));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(function(e,re) return re:IsMonsterEffect() and re:GetHandler():IsRaceExcept(RACE_ZOMBIE) end)
    effect::set_value_fn(d, e2, lua_fn(cannot_activate));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
// function(e,re) return re:IsMonsterEffect() and re:GetHandler():IsRaceExcept(RACE_ZOMBIE) end
fn cannot_activate(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_race_except(d, h, RACE_ZOMBIE, None, 0, None)
}
// Reset its type back to the original when it stops being face-up in the Monster Zone
// (the e0a and e0b of the Lua, the same in Sennet Per Akh and Zenet journey)
fn restore_type_on_leave(d: &mut Duel, c: CardId, sc: CardId, original_type: u32, tp: u8) {
    // local e0a=Effect.CreateEffect(c)
    let e0a = effect::create_effect(d, c);
    // e0a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0a:SetCode(EVENT_ADJUST)
    effect::set_code(d, e0a, EVENT_ADJUST);
    // e0a:SetCondition(function() return not sc:HasFlagEffect(id) end)
    effect::set_condition(d, e0a, Some(lua_fn(move |d, _a: Args| !card::has_flag_effect(d, sc, ID, None))));
    // e0a:SetOperation(function() sc:Type(original_type) e0a:Reset() end)
    effect::set_operation(
        d,
        e0a,
        Some(operation(move |d, _a| {
            card::info_type(d, sc, Some(original_type));
            effect::reset(d, e0a);
        })),
    );
    // Duel.RegisterEffect(e0a,tp)
    duel::register_effect(d, e0a, tp);
    // local e0b=Effect.CreateEffect(c)
    let e0b = effect::create_effect(d, c);
    // e0b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0b:SetCode(EFFECT_SEND_REPLACE)
    effect::set_code(d, e0b, EFFECT_SEND_REPLACE);
    // e0b:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e0b, Some(cost_target(move |d, a| send_replace_target(d, a, e0b))));
    // e0b:SetValue(aux.FALSE)
    effect::set_value_fn(d, e0b, aux_false());
    // Duel.RegisterEffect(e0b,tp)
    duel::register_effect(d, e0b, tp);
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) (the e0b target of restore_type_on_leave)
fn send_replace_target(d: &mut Duel, a: &EffArgs, e0b: EffectId) -> bool {
    // local g=eg:Filter(Card.HasFlagEffect,nil,id)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // for tc in g:Iter() do tc:Type(tc:GetFlagEffectLabel(id)) end
    for tc in group::iter(d, &g) {
        let label = card::get_flag_effect_label(d, tc, ID).first().copied().unwrap_or(0);
        card::info_type(d, tc, Some(label as u32));
    }
    // e0b:Reset()
    effect::reset(d, e0b);
    // return true
    true
}
