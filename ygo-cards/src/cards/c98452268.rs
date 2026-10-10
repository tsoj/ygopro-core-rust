//! Odd-Eyes Rebellion Dragon Overlord
use super::prelude::*;

pub const ID: u32 = 98452268;

pub const INFO: ScriptInfo = ScriptInfo {
    pendulum_level: Some(7),
    listed_names: &[98452268],
    listed_series: &[SET_REBELLION, SET_THE_PHANTOM_KNIGHTS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 7 monsters OR 1 "Rebellion" Xyz Monster you control
    // Xyz.AddProcedure(c,nil,7,2,function(c) return c:IsSetCard(SET_REBELLION) and c:IsType(TYPE_XYZ) and c:IsFaceup() end,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        None,
        Some(7),
        2,
        Some(xyz::alter_filter(|d, c, _tp, _xc| {
            card::is_set_card(d, c, &[SET_REBELLION as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
        })),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // You can only Special Summon "Odd-Eyes Rebellion Dragon Overlord(s)" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Special Summon this card from your Pendulum Zone, then Special Summon from your Extra Deck, 1 "Rebellion" or "The Phantom Knights" monster, using this card as material (this is treated as an Xyz Summon), then you can attach 1 card from your Pendulum Zone to it as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card that was Xyz Summoned using a Rank 7 Xyz Monster as material can make up to 3 attacks during each Battle Phase
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE);
    // e2a:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2a, EFFECT_EXTRA_ATTACK);
    // e2a:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e2a, Some(condition(has_flag_cond)));
    // e2a:SetValue(2)
    effect::set_value(d, e2a, 2);
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2b, EFFECT_TYPE_SINGLE);
    // e2b:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2b, EFFECT_MATERIAL_CHECK);
    // e2b:SetValue(s.valcheck)
    effect::set_value_fn(d, e2b, effect_card_value(valcheck));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Place this card in your Pendulum Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_MZONE) end)
    effect::set_condition(d, e3, Some(condition(has_prev_mzone_cond)));
    // e3:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return Duel.CheckPendulumZones(tp) end end)
    effect::set_target(d, e3, Some(cost_target(pen_target)));
    // e3:SetOperation(s.penop)
    effect::set_operation(d, e3, Some(operation(penop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end`
fn has_flag_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) > 0
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_MZONE) end`
fn has_prev_mzone_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `function(e,...,chk) if chk==0 then return Duel.CheckPendulumZones(tp) end end`
fn pen_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsSetCard({SET_REBELLION,SET_THE_PHANTOM_KNIGHTS}) and c:IsType(TYPE_XYZ) and c:IsFacedown()
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    //     and mc:IsCanBeXyzMaterial(c,tp) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_REBELLION as u16, SET_THE_PHANTOM_KNIGHTS as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_facedown(d, c)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && !card::is_code(d, c, &[ID])
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonCount(tp,2)
        //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and (#pg<=0 or (#pg==1 and pg:IsContains(c)))
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_count(d, tp, 2)
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,2,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 2, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !(card::is_relate_to_effect(d, c, e) && duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // if not (#pg<=0 or (#pg==1 and pg:IsContains(c))) then return end
    if !(pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c))) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c):GetFirst()
    let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sg) else {
        return;
    };
    // sc:SetMaterial(c)
    card::set_material(d, sc, CardSet::from_card(c));
    // Duel.Overlay(sc,c)
    duel::overlay(d, sc, c, false);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)==0 then return end
    if duel::special_summon(d, CardSet::from_card(sc), SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // sc:CompleteProcedure()
    card::complete_procedure(d, sc);
    // if Duel.IsExistingMatchingCard(Card.IsCanBeXyzMaterial,tp,LOCATION_PZONE,0,1,nil,c,tp,REASON_EFFECT)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::is_existing_matching_card(d, Some(&|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(tp), Some(REASON_EFFECT))), tp, LOCATION_PZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
        // local matc=Duel.SelectMatchingCard(tp,Card.IsCanBeXyzMaterial,tp,LOCATION_PZONE,0,1,1,nil,c,tp,REASON_EFFECT):GetFirst()
        let mg = duel::select_matching_card(d, tp, Some(&|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(tp), Some(REASON_EFFECT))), tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
        // if not matc then return end
        let Some(matc) = group::get_first(d, &mg) else {
            return;
        };
        // Duel.HintSelection(matc)
        duel::hint_selection(d, matc, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Overlay(sc,matc)
        duel::overlay(d, sc, matc, false);
    }
}
/// `s.valcheckfilter(c)`
fn valcheckfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRank(7) and c:IsType(TYPE_XYZ)
    card::is_rank(d, c, &[7]) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.valcheck(e,c)`: sets a flag when a Rank 7 Xyz Monster was used as material
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // if c:GetMaterial():IsExists(s.valcheckfilter,1,nil) then
    let mat = card::get_material(d, c);
    if group::is_exists(d, &mat, &|d, x| valcheckfilter(d, x), 1, NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_LEAVE|RESET_TEMP_REMOVE),EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,4))
        card::register_flag_effect(
            d,
            c,
            ID,
            RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_LEAVE | RESET_TEMP_REMOVE)),
            EFFECT_FLAG_CLIENT_HINT,
            1,
            0,
            aux::stringid(ID, 4),
        );
    }
    // the function returns nothing: the core reads nil as 0
    0
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
