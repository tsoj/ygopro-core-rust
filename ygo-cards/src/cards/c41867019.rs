//! Mayakashi Mayhem
//! Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 41867019;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_MAYAKASHI as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply 1 of the 4 different options
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_TOGRAVE+CATEGORY_DAMAGE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_TOGRAVE + CATEGORY_DAMAGE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.activate)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsType(TYPE_SYNCHRO) and not c:IsSummonLocation(LOCATION_EXTRA)
    card::is_race(d, c, RACE_ZOMBIE)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && !card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id+4)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID + 4) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id+4,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID + 4, RESET_CHAIN, 0, 1, 0);
    true
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil) and not e:GetHandler():IsStatus(STATUS_CHAINING)
    let eg = a.eg.clone().nil();
    if !group::is_exists(d, &eg, &cfilter, 1, NO_EX) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_CHAINING)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAYAKASHI) and c:IsSpellTrap() and c:IsSSetable() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_MAYAKASHI as u16])
        && card::is_spell_trap(d, c)
        && card::is_s_setable(d, c, false, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.GetFlagEffect(tp,id)==0
    //     or Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) and Duel.GetFlagEffect(tp,id+1)==0
    //     or Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) and Duel.GetFlagEffect(tp,id+2)==0 end
    if a.chk == Some(0) {
        return (duel::is_player_can_draw(d, tp, 1) && duel::get_flag_effect(d, tp, ID) == 0)
            || (duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
                && duel::get_flag_effect(d, tp, ID + 1) == 0)
            || (duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
                && duel::get_flag_effect(d, tp, ID + 2) == 0);
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(800)
    duel::set_target_param(d, 800);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,800)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 800);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,0,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 0, LOCATION_MZONE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local off=1
    // local ops={}
    // local opval={}
    let mut ops: Vec<u64> = Vec::new();
    let mut opval: Vec<i64> = Vec::new();
    // if Duel.IsPlayerCanDraw(tp,1) and Duel.GetFlagEffect(tp,id)==0 then
    if duel::is_player_can_draw(d, tp, 1) && duel::get_flag_effect(d, tp, ID) == 0 {
        // ops[off]=aux.Stringid(id,3)
        // opval[off-1]=1
        ops.push(aux::stringid(ID, 3));
        opval.push(1);
    }
    // if Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) and Duel.GetFlagEffect(tp,id+1)==0 then
    if duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::get_flag_effect(d, tp, ID + 1) == 0
    {
        ops.push(aux::stringid(ID, 4));
        opval.push(2);
    }
    // if Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) and Duel.GetFlagEffect(tp,id+2)==0 then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::get_flag_effect(d, tp, ID + 2) == 0
    {
        ops.push(aux::stringid(ID, 5));
        opval.push(3);
    }
    // if Duel.GetFlagEffect(tp,id+3)==0 then
    if duel::get_flag_effect(d, tp, ID + 3) == 0 {
        ops.push(aux::stringid(ID, 6));
        opval.push(4);
    }
    // if off==1 then return end
    if ops.is_empty() {
        return;
    }
    // local op=Duel.SelectOption(tp,table.unpack(ops))
    // (0-based result, so opval[op] below is the Lua opval[op] of the 0-based index)
    let op = duel::select_option(d, tp, true, &ops) as usize;
    let val = opval[op];
    // if opval[op]==1 then --Draw 1 card
    if val == 1 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // elseif opval[op]==2 then --Set 1 "Mayakashi" spell/trap from deck
    } else if val == 2 {
        // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.SSet(tp,sg)
            duel::s_set(d, tp, sg.clone(), None, true);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg);
            // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    // elseif opval[op]==3 then --Send opponent's monster with lowest ATK to GY
    } else if val == 3 {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // local tg=g:GetMinGroup(Card.GetAttack)
            let tg = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64).nil().0;
            // if #tg>1 then
            if tg.len() > 1 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
                // local sg=tg:Select(tp,1,1,nil)
                let sg = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
                // Duel.HintSelection(sg)
                duel::hint_selection(d, sg.clone(), true);
                // Duel.SendtoGrave(sg,REASON_EFFECT)
                duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
            // else Duel.SendtoGrave(tg,REASON_EFFECT) end
            } else {
                duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
            }
            // Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    // elseif opval[op]==4 then --Inflict 800 damage
    } else if val == 4 {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Damage(p,d,REASON_EFFECT)
        duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
        // Duel.RegisterFlagEffect(tp,id+3,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 3, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
