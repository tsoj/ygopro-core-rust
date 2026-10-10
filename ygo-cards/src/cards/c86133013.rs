//! Level Resist Wall
use super::prelude::*;

pub const ID: u32 = 86133013;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// s.tgfilter(c,e,tp,ft)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,lv)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp, lv)), tp, LOCATION_DECK, 0, NO_EX);
    // return (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp) and c:IsFaceup()
    //     and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and c:IsCanBeEffectTarget(e)
    //     and c:HasLevel() and not c:IsType(TYPE_TOKEN) and g:CheckWithSumEqual(Card.GetLevel,lv,1,ft)
    (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::has_level(d, c)
        && !card::is_type(d, c, TYPE_TOKEN)
        && group::check_with_sum_equal(d, &g, &|d, x| card::get_level(d, x) as i64, lv, 1, ft).0
}
/// s.spfilter(c,e,tp,lv)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsLevelBelow(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_level_below(d, c, lv) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=math.min(ft,1) end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = ft.min(1);
    }
    // local tg=eg:Filter(s.tgfilter,nil,e,tp,ft)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| tgfilter(d, c, e, tp, ft), NO_EX);
    // if chk==0 then return ft>0 and #tg>0 end
    if a.chk == Some(0) {
        return ft > 0 && !tg.is_empty();
    }
    // local tc=nil
    // if #tg>1 then
    let tc = if tg.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tc=tg:Select(tp,1,1,nil):GetFirst()
        let sel = group::filter_select(d, &tg, tp, &|_d: &mut Duel, _c: CardId| true, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        // tc=tg:GetFirst()
        group::get_first(d, &tg)
    };
    // Duel.SetTargetCard(tc)
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,lv)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp, lv)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:SelectWithSumEqual(tp,Card.GetLevel,lv,1,ft)
    let sg = group::select_with_sum_equal(d, &g, tp, &|d, x| card::get_level(d, x) as i64, lv, 1, ft).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for sc in sg:Iter() do
    for sc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // --Negate their effects
            // sc:NegateEffects(c)
            card::negate_effects(d, sc, c, None, false, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
