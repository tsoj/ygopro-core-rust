//! Vicious Claw
use super::prelude::*;

pub const ID: u32 = 75524092;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[75524093], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy sub
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // e4:SetOperation(s.desrepop)
    effect::set_operation(d, e4, Some(operation(desrepop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // tohand
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e5, EVENT_TO_HAND);
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tg = card::get_equip_target(d, h);
    // if chk==0 then return tg and tg:IsReason(REASON_BATTLE) end
    if a.chk == Some(0) {
        return match tg {
            Some(tg) => card::is_reason(d, tg, REASON_BATTLE),
            None => false,
        };
    }
    // return true
    true
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // local exc=e:GetHandler():GetEquipTarget():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let eqt = card::get_equip_target(d, h).nil();
    let exc = card::get_battle_target(d, eqt);
    // Duel.SendtoHand(e:GetHandler(),nil,REASON_EFFECT)
    duel::send_to_hand(d, h, None, REASON_EFFECT, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,exc)
    let ex = match exc {
        Some(x) => x.into(),
        None => NO_EX,
    };
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, ex).nil();
    // if Duel.Destroy(g,REASON_EFFECT)>0 and Duel.Damage(1-tp,600,REASON_EFFECT)~=0 then
    if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 && duel::damage(d, 1 - a.tp, 600, REASON_EFFECT, false, None) != 0 {
        // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,2500,2500,7,RACE_FIEND,ATTRIBUTE_DARK,POS_FACEUP,1-tp) then
        if duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(1 - a.tp), 0)
        {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, a.tp, ID + 1).nil();
            // Duel.SpecialSummon(token,0,tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon(d, token, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// The token of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,2500,2500,7,RACE_FIEND,ATTRIBUTE_DARK,...)`.
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(2500),
        defense: Some(2500),
        level: Some(7),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
        ..Default::default()
    }
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsPreviousLocation(LOCATION_DECK) then
    if !card::is_previous_location(d, c, LOCATION_DECK) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_USE_AS_COST)
        effect::set_code(d, e1, EFFECT_CANNOT_USE_AS_COST);
        // e1:SetTargetRange(LOCATION_HAND,0)
        effect::set_target_range(d, e1, LOCATION_HAND, 0);
        // e1:SetTarget(s.limittg)
        effect::set_target(d, e1, Some(effect_card_fn(limittg)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_CANNOT_SSET)
        effect::set_code(d, e3, EFFECT_CANNOT_SSET);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
    }
}
/// `s.limittg(e,c)`
fn limittg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsCode(id)
    card::is_code(d, c, &[ID])
}
