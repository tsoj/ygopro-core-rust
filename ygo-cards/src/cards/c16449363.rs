//! Scrap Synchron
use super::prelude::*;

pub const ID: u32 = 16449363;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_JUNK_WARRIOR],
    listed_series: &[SET_SYNCHRON, SET_WARRIOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // For a Synchro Summon, you can substitute this card for any 1 "Synchron" Tuner ("Quickdraw Synchron")
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(20932152)
    effect::set_code(d, e0, 20932152);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If you Synchro Summon a monster that mentions a "Synchron" Tuner as material, this card in your hand can also be used as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_SYNCHRO_MAT_FROM_HAND)
    effect::set_code(d, e1, EFFECT_SYNCHRO_MAT_FROM_HAND);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetValue(function(e,mc,sc) return sc:ListsArchetypeAsMaterial(SET_SYNCHRON) end)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| {
        let sc = a.card(2).nil();
        card::lists_archetype_as_material(d, sc, &[SET_SYNCHRON])
    }));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a monster(s) that mentions "Junk Warrior", and/or a Synchro Monster(s) with "Warrior" in its original name, that you control would be destroyed by battle or card effect, you can banish this card from your field or GY instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_MZONE | LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| {
        let p = effect::get_handler_player(d, e);
        repfilter(d, c, p)
    }));
    // e2:SetOperation(function(e) Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:ListsCode(CARD_JUNK_WARRIOR) or (c:IsSynchroMonster() and c:IsOriginalSetCard(SET_WARRIOR))) and c:IsFaceup() and c:IsLocation(LOCATION_MZONE)
    //   and c:IsControler(tp) and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    (card::lists_code(d, c, &[CARD_JUNK_WARRIOR]) || (card::is_synchro_monster(d, c) && card::is_original_set_card(d, c, &[SET_WARRIOR as u16])))
        && card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
// s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    //   and c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
            && card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
