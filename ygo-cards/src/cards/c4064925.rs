//! Ancient Gear Advance
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 4064925;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANCIENT_GEAR_GOLEM, ID], listed_series: &[SET_ANCIENT_GEAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 "Ancient Gear" Spell/Trap
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.drwcost)
    effect::set_cost(d, e2, Some(cost_target(drwcost)));
    // e2:SetTarget(s.drwtg)
    effect::set_target(d, e2, Some(cost_target(drwtg)));
    // e2:SetOperation(s.drwop)
    effect::set_operation(d, e2, Some(operation(drwop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register when a player Sets a card
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_MSET)
        effect::set_code(d, ge1, EVENT_MSET);
        // ge1:SetOperation(function(...) Duel.RegisterFlagEffect(rp,id,RESET_PHASE|PHASE_END,0,1) end)
        effect::set_operation(d, ge1, Some(operation(|d, a| {
            duel::register_flag_effect(d, a.rp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        })));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_SSET)
        effect::set_code(d, ge2, EVENT_SSET);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
        // local ge3=ge1:Clone()
        let ge3 = effect::clone(d, ge1);
        // ge3:SetCode(EVENT_CHANGE_POS)
        effect::set_code(d, ge3, EVENT_CHANGE_POS);
        // ge3:SetOperation(s.checkposop)
        effect::set_operation(d, ge3, Some(operation(checkposop)));
        // Duel.RegisterEffect(ge3,0)
        duel::register_effect(d, ge3, 0);
        // local ge4=ge1:Clone()
        let ge4 = effect::clone(d, ge1);
        // ge4:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge4, EVENT_SPSUMMON_SUCCESS);
        // ge4:SetOperation(s.checksumop)
        effect::set_operation(d, ge4, Some(operation(checksumop)));
        // Duel.RegisterEffect(ge4,0)
        duel::register_effect(d, ge4, 0);
    });
}
/// `s.checkposfilter(c)`
fn checkposfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsFacedown()
    card::is_previous_position(d, c, POS_FACEUP) && card::is_facedown(d, c)
}
/// `s.checkposop(e,tp,eg,ep,ev,re,r,rp)`
fn checkposop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.checkposfilter,1,nil) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| checkposfilter(d, c), 1, NO_EX) {
        // Duel.RegisterFlagEffect(rp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.rp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `s.checksumop(e,tp,eg,ep,ev,re,r,rp)`
fn checksumop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(Card.IsFacedown,1,nil) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| card::is_facedown(d, c), 1, NO_EX) {
        // Duel.RegisterFlagEffect(rp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.rp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.chk == Some(0) {
        return !duel::has_flag_effect(d, a.tp, ID, None);
    }
    // Cannot Set cards the turn you activate this card
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e1, EFFECT_CANNOT_MSET);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e2, EFFECT_CANNOT_SSET);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_CANNOT_TURN_SET)
    effect::set_code(d, e3, EFFECT_CANNOT_TURN_SET);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // local e4=e1:Clone()
    let e4 = effect::clone(d, e1);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e4:SetTarget(function(_,_,_,_,sumpos) return (sumpos&POS_FACEDOWN)>0 end)
    effect::set_target(d, e4, Some(lua_fn(|_d, a: Args| (a.int(4) as u32 & POS_FACEDOWN) > 0)));
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, a.tp);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANCIENT_GEAR) and c:IsSpellTrap() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16]) && card::is_spell_trap(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.drwcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drwcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,false,nil,nil) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, None, 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,nil,nil)
    let g = aux::select_release_group_cost(d, a.tp, None, 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.drwtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drwtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drwop(e,tp,eg,ep,ev,re,r,rp)`
fn drwop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // if Duel.Draw(p,d,REASON_EFFECT)==0 then return end
    if duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT) == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // "Ancient Gear Golem" or monsters that mention it can be Normal Summoned without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // e1:SetTarget(aux.FieldSummonProcTg(s.nttg))
    effect::set_target(d, e1, Some(aux::field_summon_proc_tg(Some(lua_fn(nttg)), None)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,4))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 4)), 0, None);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    a.int(2) == 0 && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.nttg(e,c)`
fn nttg(d: &mut Duel, a: Args) -> bool {
    // return (c:IsCode(CARD_ANCIENT_GEAR_GOLEM) or c:ListsCode(CARD_ANCIENT_GEAR_GOLEM)) and c:IsLevelAbove(5)
    let c = a.c(1);
    (card::is_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]) || card::lists_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM])) && card::is_level_above(d, c, 5)
}
