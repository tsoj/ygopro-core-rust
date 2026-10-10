//! Number 67: Pair-a-Dice Smasher
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 35772782;

// s.roll_dice=true, s.xyz_number=67
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, xyz_number: Some(67), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,5,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // roll dice
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DICE)
    effect::set_category(d, e1, CATEGORY_DICE);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // result to 7
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TOSS_DICE_NEGATE)
    effect::set_code(d, e2, EVENT_TOSS_DICE_NEGATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.dicecon)
    effect::set_condition(d, e2, Some(condition(dicecon)));
    // e2:SetOperation(s.diceop)
    effect::set_operation(d, e2, Some(operation(diceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1)
    duel::is_phase(d, PHASE_MAIN1)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, PLAYER_ALL as u8, 1);
    true
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, a.e(1))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local d1,d2,d3,d4=Duel.TossDice(tp,2,2)
    let dice = duel::toss_dice(d, tp, 2, 2);
    let (d1, d2, d3, d4) = (dice[0], dice[1], dice[2], dice[3]);
    // local t1=d1+d2
    let t1 = d1 + d2;
    // local t2=d3+d4
    let t2 = d3 + d4;
    // if t1==t2 then return end
    if t1 == t2 {
        return;
    }
    // local p=0
    // if t1>t2 then p=tp else p=1-tp end
    let p = if t1 > t2 { tp } else { 1 - tp };
    // cannot activate
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,p)
    duel::register_effect(d, e1, p);
    // cannot attack
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e2,p)
    duel::register_effect(d, e2, p);
    // client hint
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, h);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 2);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // Duel.RegisterEffect(e3,p)
    duel::register_effect(d, e3, p);
}
/// `s.dicecon(e,tp,eg,ep,ev,re,r,rp)`
fn dicecon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetOverlayCount()>0 and c:GetFlagEffect(id)==0
    card::get_overlay_count(d, c) > 0 && card::get_flag_effect(d, c, ID) == 0
}
/// `s.diceop(e,tp,eg,ep,ev,re,r,rp)`
fn diceop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    // local cid=Duel.GetChainInfo(cc,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, cc as u8, &[CHAININFO_CHAIN_ID])[0].clone();
    // if s[0]~=cid and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    // (s[0] is nil before the first use: Lua `nil ~= cid` is true)
    let same_chain = matches!((s_var(d, ID, 0), &cid), (Value::Int(prev), Value::Int(now)) if prev == *now);
    if !same_chain && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // local dc={Duel.GetDiceResult()}
        let mut dc = duel::get_dice_result(d);
        // local ac=1
        let mut ac = 1_usize;
        // local ct=(ev&0xff)+(ev>>16)
        let ct = (a.ev & 0xff) + (a.ev >> 16);
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // if ct>1 then
        if ct > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
            duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
            // local val,idx=Duel.AnnounceNumber(tp,table.unpack(dc,1,ct))
            let nums: Vec<u64> = dc.iter().take(ct as usize).map(|&x| x as u64).collect();
            let (_val, idx) = duel::announce_number(d, tp, &nums).nil();
            // ac=idx+1
            ac = idx as usize + 1;
        }
        // dc[ac]=7
        dc[ac - 1] = 7;
        // Duel.SetDiceResult(table.unpack(dc))
        let results: Vec<u8> = dc.iter().map(|&x| x as u8).collect();
        duel::set_dice_result(d, &results);
        // s[0]=cid
        set_s_var(d, ID, 0, cid);
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
