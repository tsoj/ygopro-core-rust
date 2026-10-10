//! Stoic Challenge
//! ストイック・チャレンジ
use super::prelude::*;

pub const ID: u32 = 37364101;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can only control 1 "Stoic Challenge"
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Equip only to a face-up Xyz Monster that has Xyz Material
    // aux.AddEquipProcedure(c,nil,function(c) return c:IsXyzMonster() and c:GetOverlayCount()>0 end)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_xyz_monster(d, c) && card::get_overlay_count(d, c) > 0)),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // It gains 600 ATK for each Xyz Material attached to a monster you control
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1a, EFFECT_TYPE_EQUIP);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetValue(function(e,c) return Duel.GetOverlayCount(e:GetHandlerPlayer(),1,0)*600 end)
    effect::set_value_fn(d, e1a, effect_card_value(value_atk));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // And any battle damage your opponent takes from battles involving it and their monster is doubled
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1b, EFFECT_TYPE_EQUIP);
    // e1b:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1b, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1b:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1b, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1b:SetCondition(function(e) local bc=e:GetHandler():GetEquipTarget():GetBattleTarget() return bc and bc:IsControler(1-e:GetHandlerPlayer()) end)
    effect::set_condition(d, e1b, Some(condition(cond_battle)));
    // e1b:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e1b, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // But its effects cannot be activated
    // local e1c=Effect.CreateEffect(c)
    let e1c = effect::create_effect(d, c);
    // e1c:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1c, EFFECT_TYPE_EQUIP);
    // e1c:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1c, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1c:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e1c, EFFECT_CANNOT_TRIGGER);
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // Send this card to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(cond_turn)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // When this card leaves the field, destroy the equipped monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,c) return Duel.GetOverlayCount(e:GetHandlerPlayer(),1,0)*600 end`
fn value_atk(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    duel::get_overlay_count(d, p, 1, 0, None) as i64 * 600
}
/// `function(e) local bc=e:GetHandler():GetEquipTarget():GetBattleTarget() return bc and bc:IsControler(1-e:GetHandlerPlayer()) end`
fn cond_battle(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, h).nil();
    let bc = card::get_battle_target(d, et);
    match bc {
        Some(bc) => {
            let p = effect::get_handler_player(d, a.e);
            card::is_controler(d, bc, 1 - p)
        }
        None => false,
    }
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) end`
fn cond_turn(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h);
    // if tc and tc:IsLocation(LOCATION_MZONE) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
