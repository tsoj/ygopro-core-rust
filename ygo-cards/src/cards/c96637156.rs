//! Musical Sumo Dice Games
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 96637156;

// s.roll_dice=true
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 Level 6 monsters
    // Xyz.AddProcedure(c,nil,6,2)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, None, None, false, None);
    // Roll die and move to corresponding zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_CONTROL+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_CONTROL + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE_START);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e1:SetTarget(s.mvtg)
    effect::set_target(d, e1, Some(cost_target(mvtg)));
    // e1:SetOperation(s.mvop)
    effect::set_operation(d, e1, Some(operation(mvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.mvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    true
}
/// `s.cannot_move(c)`
fn cannot_move(d: &mut Duel, c: CardId) {
    // Duel.SendtoGrave(c,REASON_EFFECT)
    duel::send_to_grave(d, c, REASON_EFFECT, None, None);
}
/// `s.mvop(e,tp,eg,ep,ev,re,r,rp)`
fn mvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsInMainMZone()) then return end
    if !(card::is_relate_to_effect(d, c, e) && card::is_in_main_m_zone(d, c, None)) {
        return;
    }
    // local dice=Duel.TossDice(tp,1)
    let dice = duel::toss_dice(d, tp, 1, 0)[0];
    // if dice<1 or dice>6 then return end
    if !(1..=6).contains(&dice) {
        return;
    }
    // local seq=c:GetSequence()
    let mut seq = card::get_sequence(d, c) as i64;
    // local col3=Duel.IsDuelType(DUEL_3_COLUMNS_FIELD)
    let col3 = duel::is_duel_type(d, DUEL_3_COLUMNS_FIELD);
    // local max=col3 and 4 or 5
    let max: i64 = if col3 { 4 } else { 5 };
    // for i=1,dice do
    for _ in 0..dice {
        // seq=seq-1
        seq -= 1;
        // if seq==0 and col3 then seq=-2
        if seq == 0 && col3 {
            seq = -2;
        // elseif seq<-max then seq=max-1 end
        } else if seq < -max {
            seq = max - 1;
        }
    }
    // local switch=seq<0
    let switch = seq < 0;
    // if switch and not c:IsAbleToChangeControler() then return s.cannot_move(c) end
    if switch && !card::is_able_to_change_controler(d, c) {
        return cannot_move(d, c);
    }
    // local fp=switch and 1-tp or tp
    let fp = if switch { 1 - tp } else { tp };
    // local nseq=switch and 5+seq or seq
    let nseq: i64 = if switch { 5 + seq } else { seq };
    // local tc=Duel.GetFieldCard(fp,LOCATION_MZONE,nseq)
    let tc = duel::get_field_card(d, fp, LOCATION_MZONE, nseq as u32);
    // if Duel.GetMZoneCount(fp,tc,tp,nil,1<<nseq)<1 then return s.cannot_move(c) end
    if duel::get_m_zone_count(d, fp, tc.into(), Some(tp), None, Some(1u32 << nseq)).0 < 1 {
        return cannot_move(d, c);
    }
    // local win=false
    let mut win = false;
    // if tc then
    if let Some(tc) = tc {
        // if c==tc or tc:IsImmuneToEffect(e) or tc:IsType(TYPE_TOKEN) or not c:IsType(TYPE_XYZ) then return s.cannot_move(c) end
        if c == tc || card::is_immune_to_effect(d, tc, e) || card::is_type(d, tc, TYPE_TOKEN) || !card::is_type(d, c, TYPE_XYZ) {
            return cannot_move(d, c);
        }
        // local prev=c:GetOverlayCount()
        let prev = card::get_overlay_count(d, c);
        // Duel.Overlay(c,tc)
        duel::overlay(d, c, tc, false);
        // win=(prev<7 and c:GetOverlayCount()>6)
        win = prev < 7 && card::get_overlay_count(d, c) > 6;
    }
    // if switch and Duel.GetControl(c,1-tp,0,0,1<<nseq) or Duel.MoveSequence(c,seq) then
    // (GetControl returns a number, which is always true in Lua)
    let moved = if switch {
        duel::get_control(d, c, 1 - tp, 0, 0, Some(1u32 << nseq), None);
        true
    } else {
        duel::move_sequence(d, c, seq as u8, None)
    };
    if moved {
        // if win then Duel.Win(tp,WIN_REASON_MUSICAL_SUMO) end
        if win {
            duel::win(d, tp, WIN_REASON_MUSICAL_SUMO);
        }
    } else {
        // else return s.cannot_move(c) end
        return cannot_move(d, c);
    }
}
