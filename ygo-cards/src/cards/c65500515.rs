//! Squirt Squid
//! 隅烏賊
use super::prelude::*;

pub const ID: u32 = 65500515;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[65500516], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // token
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetTarget(s.tktg)
    effect::set_target(d, e1, Some(cost_target(tktg)));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.tkop)
    effect::set_operation(d, e1, Some(operation(tkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be battle target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(lua_fn(tgcon)));
    // e2:SetValue(s.tgval)
    effect::set_value_fn(d, e2, effect_card_fn(tgval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,at:GetAttack(),0,2,RACE_AQUA,ATTRIBUTE_WATER)`
fn can_summon_token(d: &mut Duel, tp: u8, at: Option<CardId>) -> bool {
    let atk = card::get_attack(d, at.nil());
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(atk),
            defense: Some(0),
            level: Some(2),
            race: Some(RACE_AQUA),
            attribute: Some(ATTRIBUTE_WATER),
        },
        None,
        None,
        0,
    )
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>1 and
    //     Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,at:GetAttack(),0,2,RACE_AQUA,ATTRIBUTE_WATER)
    // end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1 && can_summon_token(d, tp, at);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if not c:IsRelateToEffect(e) or c:IsControler(1-tp) or c:IsImmuneToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if !card::is_relate_to_effect(d, c, e)
        || card::is_controler(d, c, 1 - tp)
        || card::is_immune_to_effect(d, c, e)
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
    // local seq=math.log(Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,0),2)
    let seq = (duel::select_disable_field(d, tp, 1, LOCATION_MZONE, 0, Some(0), false) as f64).log2();
    // Duel.MoveSequence(c,seq)
    duel::move_sequence(d, c, seq as u8, None);
    // if c:GetSequence()==seq and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,at:GetAttack(),0,2,RACE_AQUA,ATTRIBUTE_WATER) then
    if (card::get_sequence(d, c) as f64) == seq && can_summon_token(d, tp, at) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(at:GetAttack())
        let atk = card::get_attack(d, at.nil());
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e1)
        card::register_effect(d, token, e1, false);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.tgval(e,c)`
fn tgval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return not c:IsImmuneToEffect(e) and c:IsAttackBelow(e:GetHandler():GetDefense())
    if card::is_immune_to_effect(d, c, e) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    let def = card::get_defense(d, h);
    card::is_attack_below(d, c, def)
}
/// `s.tgcon(e)`
fn tgcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local seq=e:GetHandler():GetSequence()
    let h = effect::get_handler(d, e).nil();
    let seq = card::get_sequence(d, h);
    // return seq==0 or seq==4
    seq == 0 || seq == 4
}
