//! Maiden with Eyes of Blue
use super::prelude::*;

pub const ID: u32 = 88241506;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate attack, change battle position and special summon 1 BEWD from hand, Deck or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.natg)
    effect::set_target(d, e1, Some(cost_target(natg)));
    // e1:SetOperation(s.naop)
    effect::set_operation(d, e1, Some(operation(naop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 BEWD from hand, Deck or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e2, EVENT_BECOME_TARGET);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_BLUEEYES_W_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.natg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn natg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.naop(e,tp,eg,ep,ev,re,r,rp)`
fn naop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.NegateAttack() and c:IsRelateToEffect(e) then
    if duel::negate_attack(d) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE,POS_FACEUP_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
        duel::change_position(
            d,
            c,
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEUP_DEFENSE as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        );
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
        let nv = aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp));
        let g = duel::get_matching_group(d, Some(&nv), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g1=g:Select(tp,1,1,nil)
            let g1 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.SpecialSummon(g1,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsContains(e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_contains(&eg, h)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let nv = aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
