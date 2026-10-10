//! Chaos Angel
use super::prelude::*;

pub const ID: u32 = 22850702;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsAttribute,ATTRIBUTE_LIGHT|ATTRIBUTE_DARK),1,99,s.exmatfilter)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK, sc, st, Some(tp)))),
        1,
        99,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK, sc, st, Some(tp)))),
        None,
        None,
        None,
        None,
    );
    // Banish 1 card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // LIGHT: Synchro Monsters you control are unaffected by your opponent's activated monster effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetLabel(ATTRIBUTE_LIGHT)
    effect::set_label(d, e2, &[ATTRIBUTE_LIGHT as i64]);
    // e2:SetCondition(s.immbttlcon)
    effect::set_condition(d, e2, Some(condition(immbttlcon)));
    // e2:SetTarget(function(_,c) return c:IsType(TYPE_SYNCHRO) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_SYNCHRO))));
    // e2:SetValue(s.immval)
    effect::set_value_fn(d, e2, lua_fn(immval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // DARK: Your monsters cannot be destroyed by battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetLabel(ATTRIBUTE_DARK)
    effect::set_label(d, e3, &[ATTRIBUTE_DARK as i64]);
    // e3:SetCondition(s.immbttlcon)
    effect::set_condition(d, e3, Some(condition(immbttlcon)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Check if its Synchro Materials include LIGHT and/or DARK monsters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, effect_card_value(valcheck));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
fn immbttlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:HasFlagEffect(id) and c:GetFlagEffectLabel(id)&e:GetLabel()>0
    card::has_flag_effect(d, c, ID, None) && (card::get_flag_effect_label(d, c, ID).first().copied().nil() & effect::get_label(d, a.e)) > 0
}
fn immval(d: &mut Duel, a: Args) -> bool {
    let (e, te) = (a.e(0), a.e(1));
    // if not te:IsActivated() then return false end
    if !effect::is_activated(d, te) {
        return false;
    }
    // local trig_p,trig_typ=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_TYPE)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_TYPE]);
    let (trig_p, trig_typ) = (info[0].int(), info[1].int());
    // return trig_p==1-e:GetHandlerPlayer() and trig_typ&TYPE_MONSTER>0
    trig_p == 1 - effect::get_handler_player(d, e) as i64 && (trig_typ & TYPE_MONSTER as i64) > 0
}
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local attr=c:GetMaterial():GetBitwiseOr(Card.GetOriginalAttribute)&(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK)
    let mat = card::get_material(d, c);
    let attr = (group::get_bitwise_or(d, &mat, &|d, x| card::get_original_attribute(d, x) as i64) as u32) & (ATTRIBUTE_LIGHT | ATTRIBUTE_DARK);
    // if attr==0 then return end
    if attr == 0 {
        return 0;
    }
    // local index=(attr==ATTRIBUTE_LIGHT and 1) or (attr==ATTRIBUTE_DARK and 2) or 3
    let index = if attr == ATTRIBUTE_LIGHT {
        1
    } else if attr == ATTRIBUTE_DARK {
        2
    } else {
        3
    };
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_LEAVE|RESET_TEMP_REMOVE),EFFECT_FLAG_CLIENT_HINT,1,attr,aux.Stringid(id,index))
    card::register_flag_effect(
        d,
        c,
        ID,
        RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_LEAVE | RESET_TEMP_REMOVE)),
        EFFECT_FLAG_CLIENT_HINT,
        1,
        attr as i64,
        aux::stringid(ID, index),
    );
    0
}
