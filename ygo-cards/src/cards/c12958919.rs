//! Phantom Skyblaster
use super::prelude::*;

pub const ID: u32 = 12958919;

// s.listed_names={12958920}
// s.listed_series={SET_SKYBLASTER}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[12958920], listed_series: &[SET_SKYBLASTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon tokens
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict Damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetCost(s.damcost)
    effect::set_cost(d, e2, Some(cost_target(damcost)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,...)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:IsSetCard(SET_SKYBLASTER) then
    if card::is_set_card(d, tc, &[SET_SKYBLASTER as u16]) {
        // Duel.RegisterFlagEffect(tc:GetControler(),id,RESET_PHASE|PHASE_END,0,1)
        let p = card::get_controler(d, tc);
        duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_SKYBLASTER,TYPES_TOKEN,500,500,4,RACE_FIEND,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// The token of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_SKYBLASTER,TYPES_TOKEN,500,500,4,RACE_FIEND,ATTRIBUTE_DARK)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_SKYBLASTER as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(500),
        defense: Some(500),
        level: Some(4),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
    // if ft>ct then ft=ct end
    if ft > ct {
        ft = ct;
    }
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_SKYBLASTER,TYPES_TOKEN,500,500,4,RACE_FIEND,ATTRIBUTE_DARK) then return end
    if !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // local ctn=true
    let mut ctn = true;
    // while ft>0 and ctn do
    while ft > 0 && ctn {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // ft=ft-1
        ft -= 1;
        // if ft<=0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then ctn=false end
        if ft <= 0 || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            ctn = false;
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.damcon(e,tp,...)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// `s.damcost(e,tp,...,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetProperty(EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_OATH, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_SKYBLASTER))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_SKYBLASTER as u16]))));
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.damtg(e,tp,...,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_SKYBLASTER),tp,LOCATION_MZONE,0,nil)
    let ct = skyblaster_faceup_count(d, a.tp);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (ct * 300) as i32);
    true
}
/// `Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_SKYBLASTER),tp,LOCATION_MZONE,0,nil)`
fn skyblaster_faceup_count(d: &mut Duel, tp: u8) -> i64 {
    duel::get_matching_group_count(d, Some(&|d: &mut Duel, c: CardId| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SKYBLASTER as u16])), tp, LOCATION_MZONE, 0, NO_EX) as i64
}
/// `s.damop(e,tp,...)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_SKYBLASTER),tp,LOCATION_MZONE,0,nil)
    let ct = skyblaster_faceup_count(d, a.tp);
    // Duel.Damage(p,ct*300,REASON_EFFECT)
    duel::damage(d, p, ct * 300, REASON_EFFECT, false, None);
}
