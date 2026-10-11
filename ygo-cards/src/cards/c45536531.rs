//! Funny Dark Rabbit
// scripted by Naim
use super::prelude::*;

pub const ID: u32 = 45536531;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], listed_series: &[SET_TOON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // After this card is Normal or Special Summoned, you can Normal Summon 1 monster that mentions "Toon World" during your Main Phase this turn, in addition to your Normal Summon/Set (you can only gain this effect once per turn)
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetOperation(s.regop)
    effect::set_operation(d, e1a, Some(operation(regop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // This card is treated as a Toon monster while "Toon World" is on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e2, EFFECT_ADD_TYPE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_TOON_WORLD),0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end)
    effect::set_condition(d, e2, Some(lua_fn(|d, _a| {
        duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])),
            0,
            LOCATION_ONFIELD,
            LOCATION_ONFIELD,
            1,
            NO_EX,
        )
    })));
    // e2:SetValue(TYPE_TOON)
    effect::set_value(d, e2, TYPE_TOON as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn: You can add to your hand or place face-up on your field, 1 "Toon" Field or Continuous Spell from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.thpltg)
    effect::set_target(d, e3, Some(cost_target(thpltg)));
    // e3:SetOperation(s.thplop)
    effect::set_operation(d, e3, Some(operation(thplop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // After this card is Normal or Special Summoned, you can Normal Summon 1 monster that mentions "Toon World" during your Main Phase this turn, in addition to your Normal Summon/Set (you can only gain this effect once per turn)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:ListsCode(CARD_TOON_WORLD) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::lists_code(d, c, &[CARD_TOON_WORLD]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.thplfilter(c,tp,szone_chk)`
fn thplfilter(d: &mut Duel, c: CardId, tp: u8, szone_chk: bool) -> bool {
    // return c:IsSetCard(SET_TOON) and (c:IsContinuousSpell() or c:IsFieldSpell()) and (c:IsAbleToHand()
    //     or (c:CheckUniqueOnField(tp) and not c:IsForbidden() and (c:IsFieldSpell() or szone_chk)))
    card::is_set_card(d, c, &[SET_TOON as u16])
        && (card::is_continuous_spell(d, c) || card::is_field_spell(d, c))
        && (card::is_able_to_hand(d, c, None)
            || (card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c) && (card::is_field_spell(d, c) || szone_chk)))
}
/// `s.thpltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thpltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thplfilter,tp,LOCATION_DECK,0,1,nil,tp,Duel.GetLocationCount(tp,LOCATION_SZONE)>0) end
    if a.chk == Some(0) {
        let szone = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| thplfilter(d, c, tp, szone)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thplop(e,tp,eg,ep,ev,re,r,rp)`
fn thplop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local szone_chk=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let szone_chk = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
    // local sc=Duel.SelectMatchingCard(tp,s.thplfilter,tp,LOCATION_DECK,0,1,1,nil,tp,szone_chk):GetFirst()
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| thplfilter(d, c, tp, szone_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &sg) {
        // aux.ToHandOrElse(sc,tp,
        //     function(sc) return szone_chk or sc:IsFieldSpell() end,
        //     function(sc)
        //         if sc:IsFieldSpell() then
        //             local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
        //             if fc then
        //                 Duel.SendtoGrave(fc,REASON_RULE)
        //                 Duel.BreakEffect()
        //             end
        //             Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        //         else
        //             Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        //         end
        //     end,
        //     aux.Stringid(id,4))
        aux::to_hand_or_else(
            d,
            sc,
            tp,
            Some(&|d, c| szone_chk || card::is_field_spell(d, c)),
            Some(&|d, g| {
                let sc = group::get_first(d, g).nil();
                if card::is_field_spell(d, sc) {
                    if let Some(fc) = duel::get_field_card(d, tp, LOCATION_FZONE, 0) {
                        duel::send_to_grave(d, CardSet::from_card(fc), REASON_RULE, None, None);
                        duel::break_effect(d);
                    }
                    duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
                } else {
                    duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
                }
                0
            }),
            Some(aux::stringid(ID, 4)),
        );
    }
}
