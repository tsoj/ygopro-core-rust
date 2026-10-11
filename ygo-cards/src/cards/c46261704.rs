//! Galactikuriboh
use super::prelude::*;

pub const ID: u32 = 46261704;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_GALAXYEYES_P_DRAGON],
    listed_series: &[SET_PHOTON, SET_GALAXY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Galaxy-Eyes Photon Dragon"
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
    // e1:SetCondition(function(_,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e1, Some(condition(attacked_by_opponent)));
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destruction replacement for "Photon" or "Galaxy" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, lua_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(_,tp) return Duel.GetAttacker():IsControler(1-tp) end`
fn attacked_by_opponent(d: &mut Duel, a: &EffArgs) -> bool {
    let att = duel::get_attacker(d).nil();
    card::is_controler(d, att, 1 - a.tp)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsDiscardable() end
    if a.chk == Some(0) {
        return card::is_discardable(d, c, None, None);
    }
    // Duel.SendtoGrave(c,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, c, REASON_COST | REASON_DISCARD, None, None);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_GALAXYEYES_P_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, a.e, a.tp)),
                a.tp,
                LOCATION_HAND | LOCATION_DECK,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.matfilter(c,e)`
fn matfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and not c:IsImmuneToEffect(e)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && !card::is_immune_to_effect(d, c, e)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| spfilter(d, c, a.e, a.tp)),
        a.tp,
        LOCATION_HAND | LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local ac=Duel.GetAttacker()
        let ac = duel::get_attacker(d).nil();
        // if not (ac:IsRelateToBattle() and ac:CanAttack() and not ac:IsImmuneToEffect(e)) then return end
        if !(card::is_relate_to_battle(d, ac) && card::can_attack(d, ac) && !card::is_immune_to_effect(d, ac, a.e)) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local xg=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
        let xg = duel::get_matching_group(d, Some(&|d, x| matfilter(d, x, a.e)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if Duel.ChangeAttackTarget(g:GetFirst()) and c:HasFlagEffect(id) and c:IsLocation(LOCATION_GRAVE) and aux.nvfilter(c)
        //     and #xg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        let first = group::get_first(d, &g);
        if duel::change_attack_target(d, first, false)
            && card::has_flag_effect(d, c, ID, None)
            && card::is_location(d, c, LOCATION_GRAVE)
            && aux::nvfilter(d, c)
            && !xg.is_empty()
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
            // local tc=xg:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &xg, a.tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // Duel.Overlay(tc,c)
            duel::overlay(d, tc, c, false);
        }
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard({SET_PHOTON,SET_GALAXY}) and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    //     and not c:IsReason(REASON_REPLACE) and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_PHOTON as u16, SET_GALAXY as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
        && (card::is_reason(d, c, REASON_BATTLE)
            || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let c = a.c(1);
    let p = effect::get_handler_player(d, a.e(0));
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE)
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
