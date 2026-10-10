//! Gunkan Sushipyard Seaside Supper Spot
use super::prelude::*;

pub const ID: u32 = 62200831;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GUNKAN], listed_names: &[CARD_SUSHIP_SHARI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Place 1 "Gunkan" card from your Deck on top of your Deck
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1a, LOCATION_FZONE);
    // e1a:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e1a, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e1a:SetCondition(s.decktopcon)
    effect::set_condition(d, e1a, Some(condition(decktopcon)));
    // e1a:SetTarget(s.decktoptg)
    effect::set_target(d, e1a, Some(cost_target(decktoptg)));
    // e1a:SetOperation(s.decktopop)
    effect::set_operation(d, e1a, Some(operation(decktopop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Your opponent pays LP exactly equal to that monster's DEF in the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.xyzcon)
    effect::set_condition(d, e3, Some(condition(xyzcon)));
    // e3:SetTarget(s.xyztg)
    effect::set_target(d, e3, Some(cost_target(xyztg)));
    // e3:SetOperation(s.xyzop)
    effect::set_operation(d, e3, Some(operation(xyzop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.decktopconfilter(c,tp)`
fn decktopconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GUNKAN) and c:IsSummonPlayer(tp) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_GUNKAN as u16]) && card::is_summon_player(d, c, tp) && card::is_faceup(d, c)
}
/// `s.decktopcon(e,tp,eg,...)`
fn decktopcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.decktopconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| decktopconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.decktoptg(e,tp,...,chk)`
fn decktoptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1
    //     and Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_DECK,0,1,nil,SET_GUNKAN) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 1
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_GUNKAN as u16])), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.decktopop(e,tp,...)`
fn decktopop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,Card.IsSetCard,tp,LOCATION_DECK,0,1,1,nil,SET_GUNKAN):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_set_card(d, c, &[SET_GUNKAN as u16])), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // Duel.MoveSequence(sc,0)
        duel::move_sequence(d, sc, 0, None);
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, a.tp, 1);
    }
}
/// `s.xyzconfilter(c,tp)`
fn xyzconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:IsReasonPlayer(1-tp) and c:IsSetCard(SET_GUNKAN) and c:IsPreviousSetCard(SET_GUNKAN) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_reason_player(d, c, 1 - tp)
        && card::is_set_card(d, c, &[SET_GUNKAN as u16])
        && card::is_previous_set_card(d, c, &[SET_GUNKAN as u16])
        && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.xyzcon(e,tp,eg,...)`
fn xyzcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.xyzconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| xyzconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.xyztg(e,tp,eg,...,chk)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=eg:Filter(s.xyzconfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| xyzconfilter(d, c, a.tp), NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_SUSHIP_SHARI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_SUSHIP_SHARI]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.xyzfilter(c,e,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GUNKAN) and c:IsType(TYPE_XYZ) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)
    // (the last operand is a number, which is always true in Lua)
    card::is_set_card(d, c, &[SET_GUNKAN as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
        && {
            duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None);
            true
        }
}
/// `s.xyzop(e,tp,eg,...)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local def=0
    let mut def: i64 = 0;
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if tc:GetPreviousDefenseOnField()<0 then def=0 end
        if card::get_previous_defense_on_field(d, tc) < 0 {
            def = 0;
        }
        // def=def+tc:GetPreviousDefenseOnField()
        def += card::get_previous_defense_on_field(d, tc) as i64;
    }
    // if not Duel.CheckLPCost(1-tp,def) then return end
    if !duel::check_lp_cost(d, 1 - a.tp, def as u32) {
        return;
    }
    // Duel.PayLPCost(1-tp,def)
    duel::pay_lp_cost(d, 1 - a.tp, def as u32);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    //     and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| xyzfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if #sg>0 and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)>0 then
        if !sg.is_empty() && duel::special_summon(d, sg.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local xyzc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
            let xg = duel::select_matching_card(d, a.tp, Some(&|d, c| xyzfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            let xyzc = group::get_first(d, &xg);
            // if xyzc then
            if let Some(xyzc) = xyzc {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // xyzc:SetMaterial(sg)
                card::set_material(d, xyzc, sg.clone());
                // Duel.Overlay(xyzc,sg)
                duel::overlay(d, xyzc, sg, false);
                // if Duel.SpecialSummon(xyzc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
                if duel::special_summon(d, xyzc, SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
                    // xyzc:CompleteProcedure()
                    card::complete_procedure(d, xyzc);
                }
            }
        }
    }
}
