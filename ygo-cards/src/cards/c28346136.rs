//! Galaxy-Eyes Cipher X Dragon
use super::prelude::*;

pub const ID: u32 = 28346136;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_CIPHER_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,10,2,s.ovfilter,aux.Stringid(id,0),2,s.xyzop)
    xyz::add_procedure(d, c, None, Some(10), 2, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), Some(xyz::MaxCt::N(2)), Some(lua_fn(xyzop)), false, None);
    // Make LIGHT monsters you currently control unable to be targeted by your opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e1:SetTarget(s.cannottgtg)
    effect::set_target(d, e1, Some(cost_target(cannottgtg)));
    // e1:SetOperation(s.cannottgop)
    effect::set_operation(d, e1, Some(operation(cannottgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 Rank 9 or lower Dragon Xyz Monster from your GY to the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(lua_fn(texcon)));
    // e2:SetTarget(s.texsptg)
    effect::set_target(d, e2, Some(cost_target(texsptg)));
    // e2:SetOperation(s.texspop)
    effect::set_operation(d, e2, Some(operation(texspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsSetCard(SET_CIPHER_DRAGON,lc,SUMMON_TYPE_XYZ,tp) and c:IsFaceup()
    card::is_set_card_ctx(d, c, &[SET_CIPHER_DRAGON as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp)) && card::is_faceup(d, c)
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    // return true
    true
}
/// `s.cannottgfilter(c)`
fn cannottgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsFaceup() and not c:HasFlagEffect(id)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_faceup(d, c) && !card::has_flag_effect(d, c, ID, None)
}
/// `s.cannottgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cannottgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cannottgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cannottgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.cannottgop(e,tp,eg,ep,ev,re,r,rp)`
fn cannottgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.cannottgfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cannottgfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.IsTurnPlayer(tp) and 2 or 1
    let ct: u16 = if duel::is_turn_player(d, tp) { 2 } else { 1 };
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,ct)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, ct, 0, 0);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3061)
        effect::set_description(d, e1, 3061);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
        effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
        // e1:SetValue(aux.tgoval)
        effect::set_value_fn(d, e1, aux::tgoval());
        // e1:SetReset(RESETS_STANDARD_PHASE_END,ct)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, ct);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.texfilter(c)`
fn texfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRankBelow(9) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_XYZ) and c:IsAbleToExtra()
    card::is_rank_below(d, c, 9) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_XYZ) && card::is_able_to_extra(d, c)
}
/// `s.texsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn texsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.texfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&texfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.texcon`: `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn texcon(d: &mut Duel, a: Args) -> bool {
    duel::is_turn_player(d, a.p(1))
}
/// `s.texspop(e,tp,eg,ep,ev,re,r,rp)`
fn texspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.texfilter,tp,LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let selected = duel::select_matching_card(d, tp, Some(&texfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &selected);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc, true);
    // if not (Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_EXTRA)) then return end
    if duel::send_to_deck(d, sc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) <= 0 || !card::is_location(d, sc, LOCATION_EXTRA) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // if not (#pg<=0 or (#pg==1 and pg:IsContains(c))) then return end
    if !(pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c))) {
        return;
    }
    // if c:IsFaceup() and c:IsRelateToEffect(e) and c:IsControler(tp) and not c:IsImmuneToEffect(e)
    //     and c:IsCanBeXyzMaterial(sc,tp) and Duel.GetLocationCountFromEx(tp,tp,c,sc)>0
    //     and sc:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    let ok = card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, e)
        && card::is_controler(d, c, tp)
        && !card::is_immune_to_effect(d, c, e)
        && card::is_can_be_xyz_material(d, c, Some(sc), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(sc), None).0 > 0
        && card::is_can_be_special_summoned(d, sc, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3));
    if !ok {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // sc:SetMaterial(c)
    card::set_material(d, sc, CardSet::from_card(c));
    // Duel.Overlay(sc,c)
    duel::overlay(d, sc, c, false);
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
