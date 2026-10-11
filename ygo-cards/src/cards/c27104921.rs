//! Forge a New Future
use super::prelude::*;

pub const ID: u32 = 27104921;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x20b], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Allow counter placement and limit it to 3
    // c:EnableCounterPermit(0x20b)
    card::enable_counter_permit(d, c, 0x20b, None, None);
    // c:SetCounterLimit(0x20b,3)
    card::set_counter_limit(d, c, 0x20b, 3);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(TIMING_SPSUMMON)
    effect::set_hint_timing(d, e0, TIMING_SPSUMMON, None);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Place counter and apply effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_COUNTER + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.ctcon)
    effect::set_condition(d, e1, Some(condition(ctcon)));
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.ctconfilter(c)
fn ctconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_LINK) and c:IsLinkSummoned() and c:IsLink(4)
    card::is_type(d, c, TYPE_LINK) && card::is_link_summoned(d, c) && card::is_link(d, c, &[4])
}
// function s.ctcon(e,tp,eg,ep,ev,re,r,rp)
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.ctconfilter,1,nil)
    group::is_exists(d, &a.eg(), &ctconfilter, 1, NO_EX)
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanAddCounter(0x20b,1) and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::is_can_add_counter(d, c, 0x20b, 1, false, 0) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0x20b)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0x20b);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and c:AddCounter(0x20b,1)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::add_counter(d, c, 0x20b, 1, false)) {
        return;
    }
    // local ct=c:GetCounter(0x20b)
    let ct = card::get_counter(d, c, 0x20b);
    let tp = a.tp;
    // Declare card name and negate effects
    // if ct==1 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if ct == 1 && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
        // local ac=Duel.AnnounceCard(tp)
        let ac = duel::announce_card(d, tp, &[]);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Negate effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
        effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
        // e1:SetTarget(s.distg)
        effect::set_target(d, e1, Some(effect_card_fn(distg)));
        // e1:SetLabel(ac)
        effect::set_label(d, e1, &[ac as i64]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
        // e2:SetCondition(s.discon)
        effect::set_condition(d, e2, Some(condition(discon)));
        // e2:SetOperation(s.disop)
        effect::set_operation(d, e2, Some(operation(disop)));
        // e2:SetLabel(ac)
        effect::set_label(d, e2, &[ac as i64]);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
        effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
        // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
        effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
    }
    // Special Summon from the GY
    // elseif ct==2 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spgyfilter),tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    else if ct == 2
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, cc| spgyfilter(d, cc, a.e, tp))), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spgyfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, cc| spgyfilter(d, cc, a.e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Send self to the GY and Special Summon a Link monster
    // elseif ct==3 and c:IsAbleToGrave() and Duel.GetLocationCountFromEx(tp)>0
    //     and Duel.IsExistingMatchingCard(s.spexfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    else if ct == 3
        && card::is_able_to_grave(d, c, None)
        && duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| spexfilter(d, cc, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SendtoGrave(c,REASON_EFFECT)<1 then return end
        if duel::send_to_grave(d, c, REASON_EFFECT, None, None) < 1 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spexfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, cc| spexfilter(d, cc, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.distg(e,c)
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local code=e:GetLabel()
    let code = effect::get_label(d, e);
    // local code1,code2=c:GetOriginalCodeRule()
    let v = card::get_original_code_rule(d, c);
    let code1 = v.first().map(|x| *x as i64);
    let code2 = v.get(1).map(|x| *x as i64);
    // return code1==code or code2==code
    code1 == Some(code) || code2 == Some(code)
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e);
    // local code1,code2=re:GetHandler():GetOriginalCodeRule()
    let h = effect::get_handler(d, a.re.nil()).nil();
    let v = card::get_original_code_rule(d, h);
    let code1 = v.first().map(|x| *x as i64);
    let code2 = v.get(1).map(|x| *x as i64);
    // return code1==code or code2==code
    code1 == Some(code) || code2 == Some(code)
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
// function s.spgyfilter(c,e,tp)
fn spgyfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.spexfilter(c,e,tp)
fn spexfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLink(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_link(d, c, &[4]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
