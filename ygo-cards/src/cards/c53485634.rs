//! Red-Eyes Retro Dragon
use super::prelude::*;

pub const ID: u32 = 53485634;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RED_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Additional Normal Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(_,tp) return Duel.IsPlayerCanAdditionalSummon(tp) end)
    effect::set_condition(d, e2, Some(condition(sumcond)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sumtg)
    effect::set_target(d, e2, Some(cost_target(sumtg)));
    // e2:SetOperation(s.sumop)
    effect::set_operation(d, e2, Some(operation(sumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:GetReasonPlayer()==1-tp
    //     and c:IsReason(REASON_DESTROY) and (c:IsReason(REASON_EFFECT) or (c:IsReason(REASON_BATTLE) and Duel.GetAttacker():IsControler(1-tp)))
    //     and c:IsPreviousSetCard(SET_RED_EYES) and c:IsSetCard(SET_RED_EYES) and c:GetPreviousLevelOnField()<=7 and c:IsLevelBelow(7) and c:IsControler(tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,c:GetPreviousPosition())
    if !card::is_previous_position(d, c, POS_FACEUP) || !card::is_previous_controler(d, c, tp) || !card::is_previous_location(d, c, LOCATION_MZONE) {
        return false;
    }
    if card::get_reason_player(d, c) != 1 - tp || !card::is_reason(d, c, REASON_DESTROY) {
        return false;
    }
    let by_effect = card::is_reason(d, c, REASON_EFFECT);
    let by_battle_attack = card::is_reason(d, c, REASON_BATTLE) && {
        let attacker = duel::get_attacker(d).nil();
        card::is_controler(d, attacker, 1 - tp)
    };
    if !(by_effect || by_battle_attack) {
        return false;
    }
    if !card::is_previous_set_card(d, c, &[SET_RED_EYES as u16]) || !card::is_set_card(d, c, &[SET_RED_EYES as u16]) {
        return false;
    }
    if card::get_previous_level_on_field(d, c) > 7 || !card::is_level_below(d, c, 7) || !card::is_controler(d, c, tp) {
        return false;
    }
    let prev_pos = card::get_previous_position(d, c) as u8;
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(prev_pos), None, None)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,c:GetPreviousPosition())
    let prev_pos = card::get_previous_position(d, c) as u8;
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(prev_pos), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=eg:Filter(s.spfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, cc| spfilter(d, cc, a.e, a.tp), NO_EX);
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) and #g>0 end
    if a.chk == Some(0) {
        let spirit = duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some();
        let count = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        return !spirit
            && count > 1
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
            && !g.is_empty();
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g+c,#g+1,tp,LOCATION_HAND|LOCATION_GRAVE)
    let mut gc = g.clone();
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(gc), (g.len() + 1) as u8, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetTargetCards(e):Filter(s.filter,nil,e,tp)
    let targets = duel::get_target_cards(d, a.e).nil();
    let mut g = group::filter(d, &targets, &|d, cc| filter(d, cc, a.e, a.tp), NO_EX);
    // if Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 then
    if duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !g.is_empty()
    {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // if #g>ft then
        if g.len() as i32 > ft {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // g=g:Select(tp,ft,ft,nil)
            g = group::filter_select(d, &g, a.tp, &|_, _| true, ft as u16, ft as u16, false, NO_EX).nil();
        }
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // local pos=tc:GetPreviousPosition()
            let pos = card::get_previous_position(d, tc);
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,pos)
            duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, pos as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `function(_,tp) return Duel.IsPlayerCanAdditionalSummon(tp) end`
fn sumcond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_player_can_additional_summon(d, a.tp)
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanSummon(tp) end
    if a.chk == Some(0) {
        return duel::is_player_can_summon(d, a.tp, None);
    }
    true
}
/// `s.sumfilter`: `aux.TargetBoolFunction(Card.IsSetCard,SET_RED_EYES)`
fn sumfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_RED_EYES as u16])
}
/// `s.sumop(e,...)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_RED_EYES))
    effect::set_target(d, e1, Some(effect_card_fn(sumfilter)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
