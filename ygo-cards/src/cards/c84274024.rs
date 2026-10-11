//! Pendulum Dimension
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 84274024;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_POLYMERIZATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon - Fusion Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // e2:SetLabel(TYPE_FUSION)
    effect::set_label(d, e2, &[TYPE_FUSION as i64]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add to hand - Synchro Summon
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetTarget(s.thtg1)
    effect::set_target(d, e3, Some(cost_target(thtg1)));
    // e3:SetOperation(s.thop1)
    effect::set_operation(d, e3, Some(operation(thop1)));
    // e3:SetLabel(TYPE_SYNCHRO)
    effect::set_label(d, e3, &[TYPE_SYNCHRO as i64]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Add to hand or Special Summon - Xyz Summon
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetTarget(s.thtg2)
    effect::set_target(d, e4, Some(cost_target(thtg2)));
    // e4:SetOperation(s.thop2)
    effect::set_operation(d, e4, Some(operation(thop2)));
    // e4:SetLabel(TYPE_XYZ)
    effect::set_label(d, e4, &[TYPE_XYZ as i64]);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE, 0);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e5, EFFECT_MATERIAL_CHECK);
    // e5:SetValue(s.valcheck)
    effect::set_value_fn(d, e5, lua_fn(valcheck));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let c = a.c(1);
    // if c:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ) and c:GetMaterial():IsExists(Card.IsType,1,nil,TYPE_PENDULUM) then
    if card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ) {
        let mat = card::get_material(d, c);
        if group::is_exists(d, &mat, &|d, cc| card::is_type(d, cc, TYPE_PENDULUM), 1, NO_EX) {
            // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END&~(RESET_LEAVE|RESET_TOFIELD),0,1)
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END & !(RESET_LEAVE | RESET_TOFIELD), 0, 1, 0, 0);
        }
    }
    Value::Nil
}
// function s.cfilter(c,e,tp)
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(e:GetLabel()) and c:GetFlagEffect(id)~=0 and c:IsSummonPlayer(tp)
    let label = effect::get_label(d, e) as u32;
    card::is_faceup(d, c) && card::is_type(d, c, label) && card::get_flag_effect(d, c, ID) != 0 && card::is_summon_player(d, c, tp)
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // return #eg==1 and s.cfilter(eg:GetFirst(),e,tp)
    eg.len() == 1 && cfilter(d, group::get_first(d, &eg).nil(), a.e, a.tp)
}
// function s.spfilter(c,e,tp,lv)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // local lvl=c:GetOriginalLevel()
    let lvl = card::get_original_level(d, c);
    // return lvl>0 and lvl==lv and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    lvl > 0
        && lvl == lv
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let eg = a.eg.clone().nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,eg:GetFirst():GetOriginalLevel())
    //     and Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let lv = card::get_original_level(d, group::get_first(d, &eg).nil());
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let eg = a.eg.clone().nil();
    // if Duel.GetFlagEffect(tp,id)~=0 then return end
    if duel::get_flag_effect(d, tp, ID) != 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if eg:GetFirst():IsFaceup() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_faceup(d, group::get_first(d, &eg).nil()) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,eg:GetFirst():GetOriginalLevel()):GetFirst()
        let lv = card::get_original_level(d, group::get_first(d, &eg).nil());
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
        if let Some(tc) = tc {
            if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // tc:RegisterEffect(e1,true)
                card::register_effect(d, tc, e1, true);
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // tc:RegisterEffect(e2,true)
                card::register_effect(d, tc, e2, true);
            }
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
// function s.thfilter1(c)
fn thfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_POLYMERIZATION) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg1(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter1,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.GetFlagEffect(tp,id+1)==0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter1), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::get_flag_effect(d, tp, ID + 1) == 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop1(e,tp,eg,ep,ev,re,r,rp)
fn thop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFlagEffect(tp,id+1)~=0 then return end
    if duel::get_flag_effect(d, tp, ID + 1) != 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter1,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter1), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
    // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
}
// function s.thfilter2(c,e,tp,ft,rk)
fn thfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rk: u32) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // return c:IsLevelBelow(rk) and c:IsType(TYPE_TUNER)
    //     and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_level_below(d, c, rk)
        && card::is_type(d, c, TYPE_TUNER)
        && (card::is_able_to_hand(d, c, None)
            || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// function s.thtg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let eg = a.eg.clone().nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let _ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil,e,tp,ft,eg:GetFirst():GetRank())
    //     and Duel.GetFlagEffect(tp,id+2)==0 end
    if a.chk == Some(0) {
        let rk = card::get_rank(d, group::get_first(d, &eg).nil());
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter2(d, c, e, tp, rk)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::get_flag_effect(d, tp, ID + 2) == 0;
    }
    true
}
// function s.thop2(e,tp,eg,ep,ev,re,r,rp)
fn thop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let eg = a.eg.clone().nil();
    // if Duel.GetFlagEffect(tp,id+2)~=0 or eg:GetFirst():IsFacedown() then return end
    if duel::get_flag_effect(d, tp, ID + 2) != 0 || card::is_facedown(d, group::get_first(d, &eg).nil()) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let _ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil,e,tp,ft,eg:GetFirst():GetRank())
    let rk = card::get_rank(d, group::get_first(d, &eg).nil());
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter2(d, c, e, tp, rk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // tc=g:GetFirst()   (a global in the Lua script; a local here)
        let tc = group::get_first(d, &g).nil();
        // aux.ToHandOrElse(tc,tp,
        //     function(c) return tc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end,
        //     function(c) Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
        //     aux.Stringid(id,0))
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d, _c| {
                card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)
                    && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            }),
            Some(&|d, _g| {
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 0)),
        );
    }
    // Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
}
