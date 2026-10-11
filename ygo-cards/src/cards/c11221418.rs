//! Bujincident
use super::prelude::*;

pub const ID: u32 = 11221418;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BUJIN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BUJIN) and c:IsType(TYPE_XYZ) and c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,0,LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_BUJIN as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_able_to_remove(d, c, None, None, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, cc| card::is_able_to_hand(d, cc, None)),
            0,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            c.into(),
        )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local tg=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,g:GetFirst())
    let first = group::get_first(d, &g).nil();
    let tg = duel::get_matching_group(
        d,
        Some(&|d, cc| card::is_able_to_hand(d, cc, None)),
        tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        first.into(),
    );
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,#tg,0,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local rct=Duel.GetTurnCount(tp)+1
    let mut rct = duel::get_turn_count(d, Some(tp)) + 1;
    // if Duel.IsTurnPlayer(1-tp) then rct=rct+1 end
    if duel::is_turn_player(d, 1 - tp) {
        rct += 1;
    }
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, cc| card::is_able_to_hand(d, cc, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if #g>0 then Duel.SendtoHand(g,nil,REASON_EFFECT) end
        if !g.is_empty() {
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
        // --trigger effect
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.spresetcon)
        effect::set_condition(d, e1, Some(condition(spresetcon)));
        // e1:SetTarget(s.sptg)
        effect::set_target(d, e1, Some(cost_target(sptg)));
        // e1:SetOperation(s.spop)
        effect::set_operation(d, e1, Some(operation(spop)));
        // e1:SetLabel(rct)
        effect::set_label(d, e1, &[rct as i64]);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
    // --cannot summon/flip summon/sp summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_FLIP_SUMMON);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, tp);
    // --no damage
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e5, EFFECT_CHANGE_DAMAGE);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e5:SetTargetRange(1,1)
    effect::set_target_range(d, e5, 1, 1);
    // e5:SetValue(0)
    effect::set_value(d, e5, 0);
    // e5:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
    effect::set_reset(d, e5, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    // Duel.RegisterEffect(e5,tp)
    duel::register_effect(d, e5, tp);
    // local e6=e5:Clone()
    let e6 = effect::clone(d, e5);
    // e6:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e6, EFFECT_NO_EFFECT_DAMAGE);
    // e6:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
    effect::set_reset(d, e6, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    // Duel.RegisterEffect(e6,tp)
    duel::register_effect(d, e6, tp);
    // --player hint
    // local e7=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e7 = effect::create_effect(d, h);
    // e7:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e7, aux::stringid(ID, 2));
    // e7:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e7, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e7:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
    effect::set_reset(d, e7, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    // e7:SetTargetRange(1,1)
    effect::set_target_range(d, e7, 1, 1);
    // Duel.RegisterEffect(e7,tp)
    duel::register_effect(d, e7, tp);
    // --reset e2~e7
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e8, aux::stringid(ID, 1));
    // e8:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e8:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e8, EVENT_PHASE + PHASE_END);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetCondition(s.spresetcon)
    effect::set_condition(d, e8, Some(condition(spresetcon)));
    // e8:SetOperation(s.resetall)
    effect::set_operation(d, e8, Some(operation(resetall)));
    // e8:SetLabel(rct)
    effect::set_label(d, e8, &[rct as i64]);
    // local reset_eff_table={e2,e3,e4,e5,e6,e7}
    // e8:SetLabelObject(reset_eff_table)
    effect::set_label_object(
        d,
        e8,
        Value::Multi(vec![Value::Effect(e2), Value::Effect(e3), Value::Effect(e4), Value::Effect(e5), Value::Effect(e6), Value::Effect(e7)]),
    );
    // e8:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
    effect::set_reset(d, e8, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    // Duel.RegisterEffect(e8,tp)
    duel::register_effect(d, e8, tp);
}
/// `s.resetall(e)`
fn resetall(d: &mut Duel, a: &EffArgs) {
    // local reset_eff_table=e:GetLabelObject()
    // for _,eff in pairs(reset_eff_table) do eff:Reset() end
    if let Value::Multi(list) = effect::get_label_object(d, a.e) {
        for eff in list.iter().filter_map(|x| x.as_effect()) {
            effect::reset(d, eff);
        }
    }
    // e:Reset()
    effect::reset(d, a.e);
}
/// `s.spresetcon(e,tp,eg,ep,ev,re,r,rp)`
fn spresetcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetLabel()==Duel.GetTurnCount(tp)
    duel::is_turn_player(d, a.tp) && effect::get_label(d, a.e) == duel::get_turn_count(d, Some(a.tp)) as i64
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BUJIN) and c:IsMonster()
    card::is_set_card(d, c, &[SET_BUJIN as u16]) && card::is_monster(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.mfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && mfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectTarget(tp,s.mfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&mfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // local mc=Duel.GetFirstTarget()
    let mc = duel::first_target(d);
    // if tc:GetFlagEffect(id)>0 and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0
    //     and mc and mc:IsRelateToEffect(e) then
    let summoned = card::get_flag_effect(d, tc, ID) > 0
        && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0;
    if summoned {
        if let Some(mc) = mc {
            if card::is_relate_to_effect(d, mc, a.e) {
                // Duel.Overlay(tc,mc)
                duel::overlay(d, tc, mc, false);
            }
        }
    }
}
