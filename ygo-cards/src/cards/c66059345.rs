//! Materiactor Meltthrough
use super::prelude::*;

pub const ID: u32 = 66059345;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATERIACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top 6 cards of your Deck, and if you do, place them on top of the Deck in any order
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.excavtg)
    effect::set_target(d, e1, Some(cost_target(excavtg)));
    // e1:SetOperation(s.excavop)
    effect::set_operation(d, e1, Some(operation(excavop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Materiactor" Xyz Monster from your Extra Deck, by using 1 Level 3 Normal Monster you control as material (this is treated as an Xyz Summon)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.PayLP(1500))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1500))));
    // e2:SetTarget(s.xyztg)
    effect::set_target(d, e2, Some(cost_target(xyztg)));
    // e2:SetOperation(s.xyzop)
    effect::set_operation(d, e2, Some(operation(xyzop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach the top card of your Deck to 1 "Materiactor" Xyz Monster you control
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e3:SetCondition(function(e,tp,eg) return eg:IsExists(function(c) return c:IsXyzSummoned() and c:IsType(TYPE_XYZ) and c:IsFaceup() end,1,nil) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(s.attachtg)
    effect::set_target(d, e3, Some(cost_target(attachtg)));
    // e3:SetOperation(s.attachop)
    effect::set_operation(d, e3, Some(operation(attachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg) return eg:IsExists(function(c) return c:IsXyzSummoned() and c:IsType(TYPE_XYZ) and c:IsFaceup() end,1,nil) end`
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(
        d,
        &eg,
        &|d, c| card::is_xyz_summoned(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c),
        1,
        NO_EX,
    )
}
/// `s.excavtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn excavtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=6 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 6;
    }
    true
}
/// `s.excavop(e,tp,eg,ep,ev,re,r,rp)`
fn excavop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 then
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0 {
        // Duel.ConfirmDecktop(tp,6)
        let _ = duel::confirm_decktop(d, a.tp, 6);
        // Duel.SortDecktop(tp,tp,6)
        duel::sort_decktop(d, a.tp, a.tp, 6);
    }
}
/// `s.normalfilter(c,e,tp)`
fn normalfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsLevel(3) and c:IsType(TYPE_NORMAL) and c:IsFaceup()) then return false end
    if !(card::is_level(d, c, &[3]) && card::is_type(d, c, TYPE_NORMAL) && card::is_faceup(d, c)) {
        return false;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ as u32);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and Duel.IsExistingMatchingCard(s.xyzspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && duel::is_existing_matching_card(d, Some(&|d, cc| xyzspfilter(d, cc, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.xyzspfilter(c,e,tp,mc)`
fn xyzspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_MATERIACTOR) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and mc:IsCanBeXyzMaterial(c,tp) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_MATERIACTOR as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.normalfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| normalfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.xyzop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local mc=Duel.SelectMatchingCard(tp,s.normalfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp):GetFirst()
    let sg = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| normalfilter(d, c, e, tp)),
        tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let mc = group::get_first(d, &sg);
    // if mc then
    if let Some(mc) = mc {
        // Duel.HintSelection(mc)
        duel::hint_selection(d, CardSet::from_card(mc), true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.xyzspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,mc):GetFirst()
        let sg2 = duel::select_matching_card(
            d,
            tp,
            Some(&|d, c| xyzspfilter(d, c, e, tp, mc)),
            tp,
            LOCATION_EXTRA,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let sc = group::get_first(d, &sg2);
        // if sc then
        if let Some(sc) = sc {
            // sc:SetMaterial(mc)
            card::set_material(d, sc, CardSet::from_card(mc));
            // Duel.Overlay(sc,mc)
            duel::overlay(d, sc, CardSet::from_card(mc), false);
            // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, CardSet::from_card(sc), SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // sc:CompleteProcedure()
                card::complete_procedure(d, sc);
            }
        }
    }
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Xyz Monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_XYZ) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_XYZ))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.attachfilter(c,tp,top_c)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8, top_c: CardId) -> bool {
    // return c:IsSetCard(SET_MATERIACTOR) and c:IsType(TYPE_XYZ) and c:IsFaceup() and top_c:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_MATERIACTOR as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, top_c, Some(c), Some(tp), Some(REASON_EFFECT as u32))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let tp = a.tp;
        // local top_c=Duel.GetDecktopGroup(tp,1):GetFirst()
        let top = group::get_first(d, &duel::get_decktop_group(d, tp, 1));
        // return top_c and Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_MZONE,0,1,nil,tp,top_c)
        return match top {
            Some(top_c) => duel::is_existing_matching_card(d, Some(&|d, c| attachfilter(d, c, tp, top_c)), tp, LOCATION_MZONE, 0, 1, NO_EX),
            None => false,
        };
    }
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local top_c=Duel.GetDecktopGroup(tp,1):GetFirst()
    let top = group::get_first(d, &duel::get_decktop_group(d, tp, 1));
    // if not top_c then return end
    let Some(top_c) = top else { return };
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,4))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 4));
    // local xyzc=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_MZONE,0,1,1,nil,tp,top_c):GetFirst()
    let sg = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| attachfilter(d, c, tp, top_c)),
        tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if xyzc then
    if let Some(xyzc) = group::get_first(d, &sg) {
        // Duel.HintSelection(xyzc)
        duel::hint_selection(d, CardSet::from_card(xyzc), true);
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Overlay(xyzc,top_c)
        duel::overlay(d, xyzc, CardSet::from_card(top_c), false);
    }
}
