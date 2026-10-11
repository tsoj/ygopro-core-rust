//! Token Support
use super::prelude::*;

pub const ID: u32 = 53545926;
pub const TOKEN_ALLIANCE: u32 = ID + 1;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ALLIANCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Increase the ATK of a monster by 1000 per Token tributed also it can make up to that many attacks on monsters during each Battle Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon "Alliance Tokens" up to the number of Token destroyed during the Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and Duel.HasFlagEffect(0,id) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp) && duel::has_flag_effect(d, 0, ID, None))));
    // e2:SetTarget(s.tkntg)
    effect::set_target(d, e2, Some(cost_target(tkntg)));
    // e2:SetOperation(s.tknop)
    effect::set_operation(d, e2, Some(operation(tknop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Registers Tokens destroyed during the Battle Phase
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROYED)
        effect::set_code(d, ge1, EVENT_DESTROYED);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsBattlePhase() then return end
    if !duel::is_battle_phase(d, None) {
        return;
    }
    // local ct=eg:FilterCount(Card.IsType,nil,TYPE_TOKEN)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| card::is_type(d, c, TYPE_TOKEN), NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // for i=1,ct do Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_BATTLE,0,1) end
    for _ in 0..ct {
        let _ = duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_BATTLE, 0, 1, 0);
    }
}
/// `aux.FaceupFilter(Card.IsAttack,0)`
fn faceup_attack0(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attack(d, c, &[0])
}
/// `s.atkcostfilter(c,tp)`
fn atkcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_TOKEN) and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAttack,0),tp,LOCATION_MZONE,0,1,c)
    card::is_type(d, c, TYPE_TOKEN) && duel::is_existing_target(d, Some(&faceup_attack0), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.spcheck(sg,tp,exg)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet) -> (bool, bool) {
    // return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAttack,0),tp,LOCATION_MZONE,0,1,sg)
    (duel::is_existing_target(d, Some(&faceup_attack0), tp, LOCATION_MZONE, 0, 1, sg.into()), false)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.atkcostfilter,1,false,s.spcheck,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| atkcostfilter(d, c, tp)), 1, 1, false, Some(&spcheck), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.atkcostfilter,1,99,false,s.spcheck,nil,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| atkcostfilter(d, c, tp)), 1, 99, false, Some(&spcheck), NO_EX);
    // local ct=Duel.Release(g,REASON_COST)
    let ct = duel::release(d, g, REASON_COST, None);
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct as i64]);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() and chkc:IsAttack(0) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc) && card::is_attack(d, chkc, &[0]);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAttack,0),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_attack0), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAttack,0),tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&faceup_attack0), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,1,tp,1000*e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g, 1, tp, (1000 * label) as i32);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local ct=e:GetLabel()
        let ct = effect::get_label(d, a.e);
        let c = effect::get_handler(d, a.e).nil();
        // Gains 1000 ATK for each Token Tributed
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(ct*1000)
        effect::set_value(d, e1, ct * 1000);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // if tc:IsFaceup() then tc:RegisterEffect(e1) end
        if card::is_faceup(d, tc) {
            card::register_effect(d, tc, e1, false);
        }
        // Can make multiple attacks on monsters
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
        effect::set_code(d, e2, EFFECT_EXTRA_ATTACK_MONSTER);
        // e2:SetValue(ct-1)
        effect::set_value(d, e2, ct - 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.tkntg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkntg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ALLIANCE,0,TYPES_TOKEN,0,0,1,RACE_BEAST,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ALLIANCE,0,TYPES_TOKEN,0,0,1,RACE_BEAST,ATTRIBUTE_EARTH)`
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        TOKEN_ALLIANCE,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(1),
            race: Some(RACE_BEAST),
            attribute: Some(ATTRIBUTE_EARTH),
        },
        None,
        None,
        0,
    )
}
/// `s.tknop(e,tp,eg,ep,ev,re,r,rp)`
fn tknop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if ft <= 0 || !can_summon(d, tp) {
        return;
    }
    // if ft>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if ft > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local ct=Duel.AnnounceNumberRange(tp,1,math.min(ft,Duel.GetFlagEffect(0,id)))
    let flag = duel::get_flag_effect(d, 0, ID) as i32;
    let ct = duel::announce_number_range(d, tp, Some(1), Some(ft.min(flag) as u32), &[]).nil().0 as i64;
    // for i=1,ct do
    for _ in 0..ct {
        // local token=Duel.CreateToken(tp,TOKEN_ALLIANCE)
        let token = duel::create_token(d, tp, TOKEN_ALLIANCE);
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
