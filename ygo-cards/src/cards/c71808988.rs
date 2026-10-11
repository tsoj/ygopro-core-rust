//! Blue-Eyes Toon Ultimate Dragon
use super::prelude::*;

pub const ID: u32 = 71808988;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[53183600], listed_series: &[SET_TOON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Blue-Eyes Toon Dragon" + 2 Toon monsters
    // Fusion.AddProcMixN(c,true,true,53183600,1,s.toonmatfilter,2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (fusion::Mat::Code(53183600), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| toonmatfilter(d, x, fc, st, tp))), 2),
        ],
    );
    // Must be Special Summoned (from your Extra Deck) by shuffling the above cards from your hand, field, and/or GY into the Deck/Extra Deck
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,true)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(None),
            ..Default::default()
        },
    );
    // Your Toon monsters can attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsType(TYPE_TOON) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, x| card::is_type(d, x, TYPE_TOON))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn: You can add 1 "Toon" card, or 1 card that mentions a "Toon" card's name, from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // During damage calculation, if your Toon monster is attacked: You can banish it until the end of the Damage Step
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.toonmatfilter(c,fc,sumtype,tp)
fn toonmatfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_TOON,fc,sumtype,tp) and c:IsMonster()
    card::is_type_ctx(d, c, TYPE_TOON, Some(fc), sumtype, Some(tp)) && card::is_monster(d, c)
}
// function s.contactfil(tp)
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToDeckOrExtraAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
    duel::get_matching_group(d, Some(&card::is_able_to_deck_or_extra_as_cost), tp, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX)
}
// function s.contactop(g,tp)
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    // local fu,fd=g:Split(Card.IsFaceup,nil)
    let (fu, fd) = group::split(d, g, &|d, x| card::is_faceup(d, x), NO_EX);
    // if #fu>0 then Duel.HintSelection(fu) end
    if !fu.is_empty() {
        duel::hint_selection(d, fu, true);
    }
    // if #fd>0 then Duel.ConfirmCards(1-tp,fd) end
    if !fd.is_empty() {
        duel::confirm_cards(d, 1 - tp, fd);
    }
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_TOON) or c:ListsCodeWithArchetype(SET_TOON)) and c:IsAbleToHand()
    (card::is_set_card(d, c, &[SET_TOON as u16]) || card::lists_code_with_archetype(d, c, &[SET_TOON])) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttackTarget()
    // return bc and bc:IsControler(tp) and bc:IsType(TYPE_TOON) and bc:IsAbleToRemove()
    match duel::get_attack_target(d) {
        Some(bc) => card::is_controler(d, bc, a.tp) && card::is_type(d, bc, TYPE_TOON) && card::is_able_to_remove(d, bc, None, None, None),
        None => false,
    }
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,Duel.GetAttackTarget(),1,tp,0)
    let bc = duel::get_attack_target(d).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, bc, 1, a.tp, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // if bc and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            // local temp_banish_eff=aux.RemoveUntil(bc,nil,REASON_EFFECT,PHASE_DAMAGE,id,e,tp,aux.DefaultFieldReturnOp)
            let temp_banish_eff = aux::remove_until(d, CardSet::from_card(bc), None, REASON_EFFECT, PHASE_DAMAGE, ID, a.e, tp, Some(aux::default_field_return()), None, None, None, None, None).nil();
            // local e1=temp_banish_eff:Clone()
            let e1 = effect::clone(d, temp_banish_eff);
            // e1:SetCode(EVENT_DAMAGE_STEP_END)
            effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
            // temp_banish_eff:Reset()
            effect::reset(d, temp_banish_eff);
        }
    }
}
