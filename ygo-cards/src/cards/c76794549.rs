//! Astrograph Sorcerer
use super::prelude::*;

pub const ID: u32 = 76794549;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PENDULUM_DRAGON, SET_XYZ_DRAGON, SET_SYNCHRO_DRAGON, SET_FUSION_DRAGON],
    listed_names: &[94415058, CARD_ZARC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // pendulum set/spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rptg)
    effect::set_target(d, e1, Some(cost_target(rptg)));
    // e1:SetOperation(s.rpop)
    effect::set_operation(d, e1, Some(operation(rpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e2, aux::stringid(ID, 3));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,5))
    effect::set_description(d, e3, aux::stringid(ID, 5));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(s.hncost)
    effect::set_cost(d, e3, Some(cost_target(hncost)));
    // e3:SetTarget(s.hntg)
    effect::set_target(d, e3, Some(cost_target(hntg)));
    // e3:SetOperation(s.hnop)
    effect::set_operation(d, e3, Some(operation(hnop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    //     ge1:SetCode(EVENT_DESTROYED)
    //     ge1:SetOperation(s.checkop)
    //     Duel.RegisterEffect(ge1,0)
    // end)
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_DESTROYED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // local tc=eg:GetFirst()
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) then
        if card::is_location(d, tc, LOCATION_GRAVE | LOCATION_REMOVED) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESET_TURN_SET|RESET_TEMP_REMOVE|RESET_TOHAND|RESET_TODECK|RESET_LEAVE|RESET_TOFIELD|RESET_PHASE|PHASE_END,0,1)
            card::register_flag_effect(
                d,
                tc,
                ID,
                RESET_EVENT | RESET_TURN_SET | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_LEAVE | RESET_TOFIELD | RESET_PHASE | PHASE_END,
                0,
                1,
                0,
                0,
            );
        } else if card::is_location(d, tc, LOCATION_EXTRA) {
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
    }
}
/// `s.rpfilter(c,e,tp)`
fn rpfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(94415058) and (not c:IsForbidden()
    //     or (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_code(d, c, &[94415058])
        && (!card::is_forbidden(d, c)
            || (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.rptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rpfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rpfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
/// `s.rpop(e,tp,eg,ep,ev,re,r,rp)`
fn rpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,6))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 6));
        // local g=Duel.SelectMatchingCard(tp,s.rpfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| rpfilter(d, x, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // local op=0
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
        let op = if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None)
        {
            // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
            duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
        } else {
            // op=Duel.SelectOption(tp,aux.Stringid(id,1))
            duel::select_option(d, tp, true, &[aux::stringid(ID, 1)])
        };
        if op == 0 {
            // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        } else {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.spcfilter(c,tp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && card::is_previous_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`: `return eg:IsExists(s.spcfilter,1,nil,tp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spcfilter(d, c, tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter1(c,tp)`
fn thfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:GetFlagEffect(id)~=0 and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup())
    //     and Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil,c:GetCode())
    let code = card::get_code(d, c)[0];
    card::is_monster(d, c)
        && card::get_flag_effect(d, c, ID) != 0
        && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && duel::is_existing_matching_card(d, Some(&|d, x| thfilter2(d, x, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter2(c,code)`: `return c:IsCode(code) and c:IsAbleToHand()`
fn thfilter2(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local g=Duel.GetMatchingGroup(s.thfilter1,tp,LOCATION_GRAVE|LOCATION_REMOVED|LOCATION_EXTRA,LOCATION_GRAVE|LOCATION_REMOVED|LOCATION_EXTRA,nil,tp)
        let loc = LOCATION_GRAVE | LOCATION_REMOVED | LOCATION_EXTRA;
        let g = duel::get_matching_group(d, Some(&|d, x| thfilter1(d, x, tp)), tp, loc, loc, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,7))
            duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 7));
            // local cg=g:Select(tp,1,1,nil)
            let cg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(cg)
            duel::hint_selection(d, cg.clone(), true);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil,cg:GetFirst():GetCode())
            let first = group::get_first(d, &cg).nil();
            let code = card::get_code(d, first)[0];
            let sg = duel::select_matching_card(d, tp, Some(&|d, x| thfilter2(d, x, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg);
        }
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_PENDULUM_DRAGON) or c:IsSetCard(SET_XYZ_DRAGON) or c:IsSetCard(SET_SYNCHRO_DRAGON) or c:IsSetCard(SET_FUSION_DRAGON)) and c:IsMonster()
    //     and c:IsAbleToRemoveAsCost() and (not c:IsLocation(LOCATION_MZONE) or c:IsFaceup())
    //     and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true,true))
    (card::is_set_card(d, c, &[SET_PENDULUM_DRAGON as u16])
        || card::is_set_card(d, c, &[SET_XYZ_DRAGON as u16])
        || card::is_set_card(d, c, &[SET_SYNCHRO_DRAGON as u16])
        || card::is_set_card(d, c, &[SET_FUSION_DRAGON as u16]))
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (!card::is_location(d, c, LOCATION_MZONE) || card::is_faceup(d, c))
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, true))
}
/// `s.checkfunc = aux.PropertyTableFilter(Card.GetSetCard,SET_PENDULUM_DRAGON,SET_XYZ_DRAGON,SET_SYNCHRO_DRAGON,SET_FUSION_DRAGON)`
fn checkfunc() -> impl Fn(&mut Duel, CardId) -> Vec<i64> {
    aux::property_table_filter(
        |d, c| card::get_set_card(d, c).into_iter().map(i64::from).collect(),
        &[SET_PENDULUM_DRAGON as i64, SET_XYZ_DRAGON as i64, SET_SYNCHRO_DRAGON as i64, SET_FUSION_DRAGON as i64],
    )
}
/// `s.rescon(checkfunc)`: `function(sg,e,tp,mg)
///     if not sg:CheckDifferentProperty(checkfunc) then return false,true end
///     return Duel.IsExistingMatchingCard(s.hnfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg) end`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let checkfunc = checkfunc();
    if !group::check_different_property(d, sg, &checkfunc) {
        return (false, true);
    }
    let e = e.nil();
    (duel::is_existing_matching_card(d, Some(&|d, c| hnfilter(d, c, e, tp, Some(sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX), false)
}
/// `s.hnfilter(c,e,tp,sg)`
fn hnfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: Option<&CardSet>) -> bool {
    // return c:IsCode(CARD_ZARC) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false) and c:CheckFusionMaterial()
    //     and Duel.GetLocationCountFromEx(tp,tp,sg and (sg+e:GetHandler()) or nil,c)>0
    if !(card::is_code(d, c, &[CARD_ZARC as u32])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION as u32, tp, false, false, None, None, None)
        && card::check_fusion_material(d, c, None, None, None))
    {
        return false;
    }
    // sg and (sg+e:GetHandler()) or nil
    let ex_group = match sg {
        None => None,
        Some(sg) => {
            let h = effect::get_handler(d, e).nil();
            let mut g = sg.clone();
            group::add_card(&mut g, &CardSet::from_card(h));
            Some(g)
        }
    };
    let ex = match &ex_group {
        None => NO_EX,
        Some(g) => g.into(),
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.hncost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hncost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let mg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    // if chk==0 then return c:IsAbleToRemoveAsCost() and aux.SelectUnselectGroup(mg,e,tp,4,4,s.rescon(checkfunc),0) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && matches!(
                aux::select_unselect_group(d, &mg, Some(e), tp, Some(4), Some(4), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(mg,e,tp,4,4,s.rescon(checkfunc),1,tp,HINTMSG_REMOVE,s.rescon(checkfunc))+c
    let mut sg = aux::select_unselect_group(d, &mg, Some(e), tp, Some(4), Some(4), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), Some(&rescon), None, false)
        .unwrap_or_default();
    group::add_card(&mut sg, &CardSet::from_card(c));
    // Duel.Remove(sg,POS_FACEUP,REASON_COST)
    duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.hntg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hntg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.hnop(e,tp,eg,ep,ev,re,r,rp)`
fn hnop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.hnfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| hnfilter(d, c, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), SUMMON_TYPE_FUSION as u32, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // g:GetFirst():CompleteProcedure()
        let first = group::get_first(d, &g).nil();
        card::complete_procedure(d, first);
    }
}
