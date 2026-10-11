//! Startling Stare of the Snake-Eyes
//! 睨み統べるスネークアイズ
use super::prelude::*;

pub const ID: u32 = 74906081;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SNAKE_EYE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_SNAKE_EYE),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_SNAKE_EYE as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // return g:GetSum(Card.GetLevel)>=2
    group::get_sum(d, &g, &|d, c| card::get_level(d, c) as i64) >= 2
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // local p=c:GetOwner()
    let p = card::get_owner(d, c);
    // return c:IsFaceup() and c:IsMonster() and Duel.GetLocationCount(p,LOCATION_SZONE)>0
    //     and c:CheckUniqueOnField(p,LOCATION_SZONE)
    //     and (c:IsLocation(LOCATION_MZONE) or not c:IsForbidden())
    card::is_faceup(d, c)
        && card::is_monster(d, c)
        && duel::get_location_count(d, p, LOCATION_SZONE as u8, None, None, None).0 > 0
        && card::check_unique_on_field(d, c, p, Some(LOCATION_SZONE), None)
        && (card::is_location(d, c, LOCATION_MZONE) || !card::is_forbidden(d, c))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER) and c:IsContinuousSpell()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c)
        && card::is_original_type(d, c, TYPE_MONSTER)
        && card::is_continuous_spell(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return (e:GetLabel()==1 and chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.plfilter(chkc))
    //     or (e:GetLabel()==2 and chkc:IsLocation(LOCATION_SZONE) and s.spfilter(chkc,e,tp)) end
    if let Some(chkc) = a.chkc() {
        let label = effect::get_label(d, a.e);
        return (label == 1
            && card::is_controler(d, chkc, 1 - a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE)
            && plfilter(d, chkc))
            || (label == 2 && card::is_location(d, chkc, LOCATION_SZONE) && spfilter(d, chkc, a.e, a.tp));
    }
    // local b1=Duel.IsExistingTarget(s.plfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
    let b1 = duel::is_existing_target(d, Some(&plfilter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil,e,tp)
    let b2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(
            d,
            Some(&|d, c| spfilter(d, c, a.e, a.tp)),
            a.tp,
            LOCATION_SZONE,
            LOCATION_SZONE,
            1,
            NO_EX,
        );
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // local g=Duel.SelectTarget(tp,s.plfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&plfilter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        // if g:GetFirst():IsLocation(LOCATION_GRAVE) then
        if card::is_location(d, group::get_first(d, &g).nil(), LOCATION_GRAVE) {
            // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
            duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, 0, 0);
        }
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil,e,tp)
        let g = duel::select_target(
            d,
            a.tp,
            Some(&|d, c| spfilter(d, c, a.e, a.tp)),
            a.tp,
            LOCATION_SZONE,
            LOCATION_SZONE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Place it face-up in its owner's Spell & Trap Zone as a Continuous Spell
        // if tc:IsLocation(LOCATION_MZONE) and Duel.GetLocationCount(tc:GetOwner(),LOCATION_SZONE)==0 then
        let owner = card::get_owner(d, tc);
        if card::is_location(d, tc, LOCATION_MZONE) && duel::get_location_count(d, owner, LOCATION_SZONE as u8, None, None, None).0 == 0 {
            // Duel.SendtoGrave(tc,REASON_RULE,nil,PLAYER_NONE)
            duel::send_to_grave(d, tc, REASON_RULE, None, Some(PLAYER_NONE as u8));
        // elseif Duel.MoveToField(tc,tp,tc:GetOwner(),LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
        } else {
            let mon = card::is_monster_card(d, tc);
            if duel::move_to_field(d, tc, a.tp, owner, LOCATION_SZONE as u16, POS_FACEUP as u8, mon, None) {
                // Treated as a Continuous Spell
                // local e1=Effect.CreateEffect(e:GetHandler())
                let handler = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, handler);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_CHANGE_TYPE)
                effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
                // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
                effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
                // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
                effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    } else if op == 2 {
        // Special Summon it to your field
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
