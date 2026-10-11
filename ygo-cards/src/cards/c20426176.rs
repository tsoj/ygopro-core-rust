//! Plunder Patroll Booty
use super::prelude::*;

pub const ID: u32 = 20426176;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PLUNDER_PATROLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Change the Attribute of 1 opponent's monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.attrtg)
    effect::set_target(d, e2, Some(cost_target(attrtg)));
    // e2:SetOperation(s.attrop)
    effect::set_operation(d, e2, Some(operation(attrop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Send itself to GY if you control no "Plunder Patroll" monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.gycon)
    effect::set_condition(d, e3, Some(condition(gycon)));
    // e3:SetTarget(s.gytg)
    effect::set_target(d, e3, Some(cost_target(gytg)));
    // e3:SetOperation(s.gyop)
    effect::set_operation(d, e3, Some(operation(gyop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.attrtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attrtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsAttributeExcept(e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let label = effect::get_label(d, a.e) as u32;
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && card::is_faceup(d, chkc)
            && card::is_attribute_except(d, chkc, label, None, 0, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanBeEffectTarget,e),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_can_be_effect_target(d, c, Some(a.e)))),
        a.tp,
        0,
        LOCATION_MZONE,
        NO_EX,
    );
    // local att=Duel.AnnounceAnotherAttribute(g,tp)
    let att = duel::announce_another_attribute(d, &g, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local sel=g:FilterSelect(tp,Card.IsAttributeExcept,1,1,nil,att)
    let sel = group::filter_select(d, &g, a.tp, &|d, c| card::is_attribute_except(d, c, att, None, 0, None), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(sel)
    duel::set_target_card(d, sel);
    // e:SetLabel(att)
    effect::set_label(d, a.e, &[att as i64]);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.tdspfilter(c,e,tp,ft)`
fn tdspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsSetCard(SET_PLUNDER_PATROLL) and c:IsMonster()
    //     and (c:IsAbleToDeck() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_deck(d, c) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.attrop(e,tp,eg,ep,ev,re,r,rp)`
fn attrop(d: &mut Duel, a: &EffArgs) {
    // local attr=e:GetLabel()
    let attr = effect::get_label(d, a.e) as u32;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsAttributeExcept(attr)) or tc:IsImmuneToEffect(e) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_attribute_except(d, tc, attr, None, 0, None))
        || card::is_immune_to_effect(d, tc, a.e)
    {
        return;
    }
    // --Change its Attribute
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetValue(attr)
    effect::set_value(d, e1, attr as i64);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.tdspfilter),tp,LOCATION_GRAVE,0,nil,e,tp,ft)
    let g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| tdspfilter(d, c, a.e, a.tp, ft))),
        a.tp,
        LOCATION_GRAVE,
        0,
        NO_EX,
    );
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // local b1=sc:IsAbleToDeck()
        let b1 = card::is_able_to_deck(d, sc);
        // local b2=ft>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        let b2 = ft > 0 && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None);
        // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,4)}, {b2,aux.Stringid(id,5)})
        let Some(op) = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]) else {
            // if not op then return end
            return;
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        if op == 1 {
            // Duel.HintSelection(sc,true)
            duel::hint_selection(d, sc, true);
            // Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, sc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        } else {
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.gycon(e,tp,eg,ep,ev,re,r,rp)`
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PLUNDER_PATROLL),tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    }
}
