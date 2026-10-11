//! Foolish Trap Hole
// scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 10529441;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When a monster effect is activated on your opponent's field during the turn they Special Summoned a monster(s): Destroy the monster that activated that effect, then destroy all opponent's cards in its adjacent Monster Zones and Spell & Trap Zones (if any)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of a player Special Summoning a monster
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        // ge1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
        effect::set_operation(d, ge1, Some(operation(global_op)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// The operation of `ge1`: `for p=0,1 do if eg:IsExists(Card.IsSummonPlayer,1,nil,p) then Duel.RegisterFlagEffect(p,id,RESET_PHASE|PHASE_END,0,1) end end`
fn global_op(d: &mut Duel, a: &EffArgs) {
    for p in 0..2u8 {
        let eg = a.eg.clone().nil();
        if group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, p), 1, NO_EX) {
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, ev) = (a.tp, a.ev as u8);
    // if not (Duel.HasFlagEffect(1-tp,id) and re:IsMonsterEffect()) then return false end
    if !(duel::has_flag_effect(d, 1 - tp, ID, None) && effect::is_monster_effect(d, a.re.nil())) {
        return false;
    }
    // local trig_loc,trig_ctrl=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_CONTROLER)
    let info = duel::get_chain_info(d, ev, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_CONTROLER]);
    let trig_loc = info[0].clone();
    let trig_ctrl = info[1].clone();
    // return trig_ctrl==1-tp and trig_loc==LOCATION_MZONE and re:GetHandler():IsRelateToEffect(re)
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    matches!(trig_ctrl, Value::Int(v) if v == (1 - tp) as i64)
        && matches!(trig_loc, Value::Int(v) if v == LOCATION_MZONE as i64)
        && card::is_relate_to_effect(d, h, re)
}
/// `s.adjacentfilter(c,tp,rc,seq)`
fn adjacentfilter(d: &mut Duel, c: CardId, tp: u8, rc: CardId, seq: i64) -> bool {
    // if c:IsControler(tp) then return false end
    if card::is_controler(d, c, tp) {
        return false;
    }
    // if c:IsLocation(LOCATION_SZONE) then
    if card::is_location(d, c, LOCATION_SZONE) {
        // return rc:IsInMainMZone() and rc:GetColumnGroup():IsContains(c) and c:IsControler(rc:GetControler())
        if !card::is_in_main_m_zone(d, rc, None) {
            return false;
        }
        let col = card::get_column_group(d, rc, 0, 0);
        return group::is_contains(&col, c) && card::is_controler(d, c, card::get_controler(d, rc));
    } else if card::is_location(d, c, LOCATION_MZONE) {
        // if c:IsInExtraMZone() or rc:IsInExtraMZone() then
        if card::is_in_extra_m_zone(d, c, None) || card::is_in_extra_m_zone(d, rc, None) {
            // return rc:GetColumnGroup():IsContains(c)
            let col = card::get_column_group(d, rc, 0, 0);
            return group::is_contains(&col, c);
        } else {
            // return c:IsSequence(seq-1,seq+1) and c:IsControler(rc:GetControler())
            return card::is_sequence(d, c, &[(seq - 1) as u32, (seq + 1) as u32]) && card::is_controler(d, c, card::get_controler(d, rc));
        }
    }
    // return false
    false
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // rc:CreateEffectRelation(e)
    card::create_effect_relation(d, rc, a.e);
    // local g=rc:GetColumnGroup(1,1):Match(s.adjacentfilter,nil,tp,rc,rc:GetSequence())
    let seq = card::get_sequence(d, rc) as i64;
    let mut g = card::get_column_group(d, rc, 1, 1);
    group::match_(d, &mut g, &|d, c| adjacentfilter(d, c, tp, rc, seq), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g+rc,#g+1,tp,0)
    let n = (g.len() + 1) as u8;
    let mut gr = g.clone();
    group::add_card(&mut gr, &CardSet::from_card(rc));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(gr), n, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (tp, re) = (a.tp, a.re.nil());
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // if not rc:IsRelateToEffect(re) then return end
    if !card::is_relate_to_effect(d, rc, re) {
        return;
    }
    // local g=rc:GetColumnGroup(1,1):Match(s.adjacentfilter,nil,tp,rc,rc:GetSequence())
    let seq = card::get_sequence(d, rc) as i64;
    let mut g = card::get_column_group(d, rc, 1, 1);
    group::match_(d, &mut g, &|d, c| adjacentfilter(d, c, tp, rc, seq), NO_EX);
    // if Duel.Destroy(rc,REASON_EFFECT)>0 and #g>0 then
    if duel::destroy(d, rc, REASON_EFFECT, None, None) > 0 && !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
