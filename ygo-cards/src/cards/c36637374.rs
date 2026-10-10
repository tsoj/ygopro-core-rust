//! Branded Opening
//! Script by XyLeN
use super::prelude::*;

pub const ID: u32 = 36637374;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DESPIA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e,tp,ft)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_DESPIA) and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)))
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_DESPIA as u16])
        && (card::is_able_to_hand(d, c, None)
            || (ft > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,ft)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler(),REASON_EFFECT) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, ft)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)),
                tp,
                LOCATION_HAND,
                0,
                1,
                h.into(),
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)>0 then
    if duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp,ft)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp, ft)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local sc=g:GetFirst()
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // aux.ToHandOrElse(sc,tp,function(c) ... end, function(c) Duel.SpecialSummon(...) end, aux.Stringid(id,1))
            aux::to_hand_or_else(
                d,
                sc,
                tp,
                Some(&|d, _c| {
                    card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None) && ft > 0
                }),
                Some(&|d, _g| {
                    duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
                    0
                }),
                Some(aux::stringid(ID, 1)),
            );
        }
        // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
        if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetTarget(s.splimit)
            effect::set_target(d, e1, Some(lua_fn(splimit)));
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
    }
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION)
    card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_FUSION as u32)
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsType(TYPE_FUSION) and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_type(d, c, TYPE_FUSION as u32)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 and e:GetHandler():IsAbleToRemove()
    //     and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return duel::get_flag_effect(d, tp, ID) == 0
            && card::is_able_to_remove(d, h, None, None, None)
            && group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
