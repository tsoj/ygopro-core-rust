//! Gunkan Suship Shirauo-class Carrier
use super::prelude::*;

pub const ID: u32 = 21293424;

const CARD_SUSHIP_SHIRAUO: u32 = 78362751;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GUNKAN],
    listed_names: &[CARD_SUSHIP_SHARI, CARD_SUSHIP_SHIRAUO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon.
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // "Gunkan" monsters cannot be destroyed by opponent's effects
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e0, EFFECT_INDESTRUCTABLE_EFFECT);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e0, LOCATION_MZONE, 0);
    // e0:SetCondition(s.contcon)
    effect::set_condition(d, e0, Some(condition(contcon)));
    // e0:SetTarget(s.conttg)
    effect::set_target(d, e0, Some(effect_card_fn(conttg)));
    // e0:SetValue(aux.indoval)
    effect::set_value_fn(d, e0, aux::indoval());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // "Gunkan" monsters gain ATK equal to their original DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.contcon)
    effect::set_condition(d, e1, Some(condition(contcon)));
    // e1:SetTarget(s.conttg)
    effect::set_target(d, e1, Some(effect_card_fn(conttg)));
    // e1:SetValue(function(_,c)return c:GetBaseDefense() end)
    effect::set_value_fn(d, e1, effect_card_value(basedef_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains effects based on material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetLabel(0)
    effect::set_label(d, e2, &[0]);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetTarget(s.regtg)
    effect::set_target(d, e2, Some(cost_target(regtg)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, effect_card_value(valcheck));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.contcon(e)`
fn contcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `s.conttg(e,c)`
fn conttg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_GUNKAN) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_set_card(d, c, &[SET_GUNKAN as u16]) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `function(_,c)return c:GetBaseDefense() end`
fn basedef_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_base_defense(d, c) as i64
}
/// `s.regcon(e,tp,...)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned() and e:GetLabel()>0
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h) && effect::get_label(d, a.e) > 0
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GUNKAN) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GUNKAN as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.regtg(e,tp,...,chk)`
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local effs=e:GetLabel()
    let effs = effect::get_label(d, a.e);
    // if chk==0 then return ((effs&1)>0 and Duel.IsPlayerCanDraw(tp,1))
    //     or ((effs&2)>0 and Duel.IsExistingMatchingCard(s.sfilter,tp,LOCATION_DECK,0,1,nil)) end
    if a.chk == Some(0) {
        return ((effs & 1) > 0 && duel::is_player_can_draw(d, a.tp, 1))
            || ((effs & 2) > 0 && duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX));
    }
    // local cat=0
    let mut cat: u64 = 0;
    // if (effs&1)>0 then
    if (effs & 1) > 0 {
        // cat=CATEGORY_DRAW
        cat = CATEGORY_DRAW;
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    // if (effs&2)>0 then
    if (effs & 2) > 0 {
        // cat=cat|CATEGORY_TOHAND|CATEGORY_SEARCH
        cat = cat | CATEGORY_TOHAND | CATEGORY_SEARCH;
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    // e:SetCategory(cat)
    effect::set_category(d, a.e, cat);
    true
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local effs=e:GetLabel()
    let effs = effect::get_label(d, a.e);
    // "Gukan Suship Shari": Draw 1 card.
    // if (effs&1)>0 then
    if (effs & 1) > 0 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
    // "Gukan Suship Shirauo": Add 1 "Gunkan" Spell/Trap from your Deck to your hand.
    // if (effs&2)>0 then
    if (effs & 2) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.sfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local effs=0
    let mut effs: i64 = 0;
    // if g:IsExists(Card.IsCode,1,nil,CARD_SUSHIP_SHARI) then effs=effs|1 end
    if group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[CARD_SUSHIP_SHARI]), 1, NO_EX) {
        effs |= 1;
    }
    // if g:IsExists(Card.IsCode,1,nil,CARD_SUSHIP_SHIRAUO) then effs=effs|2 end
    if group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[CARD_SUSHIP_SHIRAUO]), 1, NO_EX) {
        effs |= 2;
    }
    // e:GetLabelObject():SetLabel(effs)
    let e2 = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, e2, &[effs]);
    0
}
