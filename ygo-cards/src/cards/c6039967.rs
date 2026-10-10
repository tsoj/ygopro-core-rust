//! Vampire Fraulein
use super::prelude::*;

pub const ID: u32 = 6039967;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg1)
    effect::set_target(d, e1, Some(cost_target(sptg1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(operation(spop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK/DEF per multiples of 100
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetCost(s.atkcost)
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register when destroying a monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special summon the monsters this card destroyed by battle
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e4, EVENT_PHASE | PHASE_BATTLE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.spcon2)
    effect::set_condition(d, e4, Some(condition(spcon2)));
    // e4:SetTarget(s.sptg2)
    effect::set_target(d, e4, Some(cost_target(sptg2)));
    // e4:SetOperation(s.spop2)
    effect::set_operation(d, e4, Some(operation(spop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.GetAttackTarget()~=nil then
    if duel::get_attack_target(d).is_some() {
        // return (Duel.GetAttacker():IsControler(tp) and Duel.GetAttacker():IsRace(RACE_ZOMBIE))
        //     or (Duel.GetAttackTarget() and Duel.GetAttackTarget():IsControler(tp) and Duel.GetAttackTarget():IsRace(RACE_ZOMBIE))
        let atk = duel::get_attacker(d).nil();
        if card::is_controler(d, atk, a.tp) && card::is_race(d, atk, RACE_ZOMBIE) {
            return true;
        }
        if let Some(at) = duel::get_attack_target(d) {
            return card::is_controler(d, at, a.tp) && card::is_race(d, at, RACE_ZOMBIE);
        }
        return false;
    }
    false
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return e:GetHandler():GetFlagEffect(id)==0 and Duel.CheckLPCost(tp,100)
        return card::get_flag_effect(d, h, ID) == 0 && duel::check_lp_cost(d, a.tp, 100);
    }
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, a.tp);
    // local m=math.floor(math.min(lp,3000)/100)
    let m = lp.min(3000).div_euclid(100);
    // local t={}
    // for i=1,m do t[i]=i*100 end
    let t: Vec<u64> = (1..=m).map(|i| (i * 100) as u64).collect();
    // local ac=Duel.AnnounceNumber(tp,table.unpack(t))
    let ac = duel::announce_number(d, a.tp, &t).nil().0;
    // Duel.PayLPCost(tp,ac)
    duel::pay_lp_cost(d, a.tp, ac as u32);
    // e:SetLabel(ac)
    effect::set_label(d, a.e, &[ac as i64]);
    // e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE_CAL,0,1)
    card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_DAMAGE_CAL, 0, 1, 0, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=Duel.GetAttacker()
    let mut c = duel::get_attacker(d).nil();
    // if c:IsControler(1-tp) then c=Duel.GetAttackTarget() end
    if card::is_controler(d, c, 1 - a.tp) {
        c = duel::get_attack_target(d).nil();
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
    // e1:SetValue(e:GetLabel())
    let lbl = effect::get_label(d, a.e);
    effect::set_value(d, e1, lbl);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 0, 1, 0, 0);
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id+1)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID + 1) != 0
}
/// `s.spfilter2(c,e,tp,rc,tid)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: CardId, tid: i32) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:GetReasonCard()==rc and c:GetTurnID()==tid
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_reason(d, c, REASON_BATTLE)
        && card::get_reason_card(d, c) == Some(rc)
        && card::get_turn_id(d, c) as i32 == tid
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let tid = duel::get_turn_count(d, None);
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp,e:GetHandler(),Duel.GetTurnCount())
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, a.e, a.tp, h, tid)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,e:GetHandler(),Duel.GetTurnCount())
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter2(d, c, a.e, a.tp, h, tid)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, LOCATION_GRAVE as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local g=nil
    // local tg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter2),tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,e:GetHandler(),Duel.GetTurnCount())
    let h = effect::get_handler(d, a.e).nil();
    let tid = duel::get_turn_count(d, None);
    let tg = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| spfilter2(d, c, a.e, a.tp, h, tid))),
        a.tp,
        LOCATION_GRAVE,
        LOCATION_GRAVE,
        NO_EX,
    );
    // if #tg>ft then
    let g = if tg.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // g=tg:Select(tp,ft,ft,nil)
        group::select(d, &tg, a.tp, ft as u16, ft as u16, false, NO_EX).nil()
    } else {
        // g=tg
        tg
    };
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
