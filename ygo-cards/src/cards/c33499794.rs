//! Nemleria Repeter
use super::prelude::*;

pub const ID: u32 = 33499794;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DREAMING_NEMLERIA], listed_series: &[SET_NEMLERIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of the effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(condition_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_NEMLERIA),tp,LOCATION_ONFIELD,0,1,e:GetHandler())
    let f = aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_NEMLERIA as u16]));
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_ONFIELD, 0, 1, h.into())
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemoveAsCost(POS_FACEDOWN) and not c:IsCode(CARD_DREAMING_NEMLERIA)
    card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8)) && !card::is_code(d, c, &[CARD_DREAMING_NEMLERIA])
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEMLERIA) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_NEMLERIA as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(10) and c:IsRace(RACE_BEAST) and c:IsFaceup() and c:IsAbleToGrave()
    card::is_level(d, c, &[10]) && card::is_race(d, c, RACE_BEAST as u64) && card::is_faceup(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_EXTRA,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, x| cfilter(d, x)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // local b1=ct>=1 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
    let b1 = ct >= 1 && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b2=ct>=2 and not Duel.HasFlagEffect(tp,id)
    let b2 = ct >= 2 && !duel::has_flag_effect(d, a.tp, ID, None);
    // local b3=ct>=3 and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil)
    let b3 = ct >= 3
        && duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_negatable_monster(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).unwrap_or(0);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_EXTRA,0,op,op,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x)), a.tp, LOCATION_EXTRA, 0, op as u16, op as u16, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEDOWN,REASON_COST)
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_COST, None, None);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    // elseif op==3 then
    } else if op == 3 {
        // e:SetCategory(CATEGORY_TOGRAVE|CATEGORY_DISABLE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE | CATEGORY_DISABLE);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_MZONE as i32);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    }
    true
}
/// `function(e,re,val,r,rp,rc) return math.floor(val/2) end`
fn halve_damage(_d: &mut Duel, a: Args) -> i64 {
    // math.floor(val/2): val is the third argument
    a.int(2).div_euclid(2)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if op == 1 {
        // Add 1 "Nemleria" card from your GY to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter(d, x)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    } else if op == 2 {
        // if Duel.HasFlagEffect(tp,id) then return end
        if duel::has_flag_effect(d, a.tp, ID, None) {
            return;
        }
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Any damage you take this turn is halved
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,4))
        effect::set_description(d, e1, aux::stringid(ID, 4));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CHANGE_DAMAGE)
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(function(e,re,val,r,rp,rc) return math.floor(val/2) end)
        effect::set_value_fn(d, e1, lua_fn(halve_damage));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    } else if op == 3 {
        // Send 1 Level 10 Beast monster to the GY and negate the effects of monsters your opponent controls
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local tgc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| tgfilter(d, x)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let tgc = group::get_first(d, &sel);
        // if tgc and Duel.SendtoGrave(tgc,REASON_EFFECT)>0 and tgc:IsLocation(LOCATION_GRAVE) then
        if let Some(tgc) = tgc {
            if duel::send_to_grave(d, CardSet::from_card(tgc), REASON_EFFECT, None, None) > 0 && card::is_location(d, tgc, LOCATION_GRAVE) {
                // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil)
                let g = duel::get_matching_group(d, Some(&|d, x| card::is_negatable_monster(d, x)), a.tp, 0, LOCATION_MZONE, NO_EX);
                // for tc in g:Iter() do
                for tc in group::iter(d, &g) {
                    // tc:NegateEffects(c,RESET_PHASE|PHASE_END)
                    card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
                }
            }
        }
    }
}
