//! Power Patron DoomZ
use super::prelude::*;

pub const ID: u32 = 81650695;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[68231287], listed_series: &[SET_DOOMZ as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You cannot Special Summon from the Extra Deck, except Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_XYZ) end)
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_XYZ))),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.addContinuousLizardCheck(c,LOCATION_MZONE,function(e,c) return not c:IsOriginalType(TYPE_XYZ) end)
    aux::add_continuous_lizard_check(
        d,
        c,
        LOCATION_MZONE,
        Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_XYZ))),
        None,
        None,
    );
    // Special Summon from your Extra Deck, 1 "DoomZ" Xyz Monster or "Jupiter the Power Patron of Destruction" by using another Effect monster as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "DoomZ" card from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.xyzmatfilter(c,e,tp)`
fn xyzmatfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local mustg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let mustg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return #mustg<=1 and c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:HasLevel()
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,c:GetLevel(),mustg)
    if mustg.len() > 1 || !card::is_faceup(d, c) || !card::is_type(d, c, TYPE_EFFECT) || !card::has_level(d, c) {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c, lv, &mustg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,mc,lv,mustg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, lv: u32, mustg: &CardSet) -> bool {
    // return c:IsType(TYPE_XYZ) and (c:IsSetCard(SET_DOOMZ) or c:IsCode(68231287))
    //   and c:IsRank(lv) and mc:IsCanBeXyzMaterial(c,tp) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    //   and (#mustg<=0 or mustg:IsContains(mc)) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    card::is_type(d, c, TYPE_XYZ)
        && (card::is_set_card(d, c, &[SET_DOOMZ as u16]) || card::is_code(d, c, &[68231287]))
        && card::is_rank(d, c, &[lv])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
        && (mustg.is_empty() || group::is_contains(mustg, mc))
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc~=c and s.xyzmatfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && chkc != c && xyzmatfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzmatfilter,tp,LOCATION_MZONE,0,1,c,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| xyzmatfilter(d, x, a.e, tp)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xyzmatfilter,tp,LOCATION_MZONE,0,1,1,c,e,tp)
    let _ = duel::select_target(d, tp, Some(&|d, x| xyzmatfilter(d, x, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, tp, LOCATION_MZONE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) and not tc:IsImmuneToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && card::is_controler(d, tc, tp)
        && !card::is_immune_to_effect(d, tc, a.e))
    {
        return;
    }
    // local mustg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let mustg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc)), tp, None, None, REASON_XYZ);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetLevel(),mustg):GetFirst()
    let lv = card::get_level(d, tc);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| spfilter(d, x, a.e, tp, tc, lv, &mustg)),
        tp,
        LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, tc, false);
        // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)==0 then return end
        if duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and c:IsControler(tp) and Duel.Equip(tp,c,sc) then
        if card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, tp) && duel::equip(d, tp, c, sc, true, false) {
            // --Equip limit
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DOOMZ) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DOOMZ as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
