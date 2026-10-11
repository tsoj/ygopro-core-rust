//! Marincess Snow
use super::prelude::*;

pub const ID: u32 = 80627281;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:GetPreviousControler()==tp and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsPreviousSetCard(SET_MARINCESS) and c:GetPreviousTypeOnField()&TYPE_LINK==TYPE_LINK
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsCanBeEffectTarget(e) and c:GetLink()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c:GetLink())
    if !(card::is_previous_position(d, c, POS_FACEUP)
        && card::get_previous_controler(d, c) == tp
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_set_card(d, c, &[SET_MARINCESS as u16])
        && (card::get_previous_type_on_field(d, c) & TYPE_LINK) == TYPE_LINK
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::get_link(d, c) != 0)
    {
        return false;
    }
    let lk = card::get_link(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, lk)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,lk)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lk: u32) -> bool {
    // return c:IsSetCard(SET_MARINCESS) and c:IsLinkMonster() and c:GetLink()<lk
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_LINK,tp,false,false)
    if !(card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_link_monster(d, c) && card::get_link(d, c) < lk) {
        return false;
    }
    // Duel.GetLocationCountFromEx(...) is a number, so it is always true as a condition: the call is kept
    let _ = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None);
    card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_LINK, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg();
    // if chkc then return eg:IsContains(chkc) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&eg, chkc) && filter(d, chkc, e, tp);
    }
    // if chk==0 then return eg:IsExists(s.filter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| filter(d, c, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=eg:FilterSelect(tp,s.filter,1,1,nil,e,tp)
    let g = group::filter_select(d, &eg, tp, &|d, c| filter(d, c, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.GetLocationCountFromEx(tp)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let lk = card::get_link(d, tc);
            let e = a.e;
            // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc:GetLink()):GetFirst()
            let sel = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp, lk)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel);
            // if sc and Duel.SpecialSummonStep(sc,SUMMON_TYPE_LINK,tp,tp,false,false,POS_FACEUP) then
            if let Some(sc) = sc {
                if duel::special_summon_step(d, sc, SUMMON_TYPE_LINK, tp, tp, false, false, POS_FACEUP as u8, None) {
                    // Unaffected by opponent's card effects
                    // local e1=Effect.CreateEffect(c)
                    let e1 = effect::create_effect(d, c);
                    // e1:SetDescription(3110)
                    effect::set_description(d, e1, 3110);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
                    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
                    // e1:SetRange(LOCATION_MZONE)
                    effect::set_range(d, e1, LOCATION_MZONE);
                    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
                    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
                    // e1:SetValue(s.efilter)
                    effect::set_value_fn(d, e1, lua_fn(efilter));
                    // e1:SetOwnerPlayer(tp)
                    effect::set_owner_player(d, e1, tp);
                    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                    // sc:RegisterEffect(e1,true)
                    card::register_effect(d, sc, e1, true);
                    // sc:CompleteProcedure()
                    card::complete_procedure(d, sc);
                }
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
// function s.efilter(e,re)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetOwnerPlayer()~=re:GetOwnerPlayer()
    effect::get_owner_player(d, a.e(0)) != effect::get_owner_player(d, a.e(1))
}
