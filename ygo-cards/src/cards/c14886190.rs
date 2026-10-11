//! Virtual World City - Kauwloon
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 14886190;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VIRTUAL_WORLD_GATE, SET_VIRTUAL_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DECKDES+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DECKDES + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.tftg)
    effect::set_target(d, e1, Some(cost_target(tftg)));
    // e1:SetOperation(s.tfop)
    effect::set_operation(d, e1, Some(operation(tfop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tffilter(c,tp)`
fn tffilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsSetCard(SET_VIRTUAL_WORLD_GATE) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_spell_trap(d, c)
        && card::is_set_card(d, c, &[SET_VIRTUAL_WORLD_GATE as u16])
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, Some(LOCATION_ONFIELD), None)
}
/// `s.tftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.tffilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, c| tffilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,3)
    duel::set_possible_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 3);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VIRTUAL_WORLD) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.exfilter1(c)`
fn exfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsType(TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    card::is_facedown(d, c) && card::is_type(d, c, TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ)
}
/// `s.exfilter2(c)`
fn exfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_LINK) or (c:IsFaceup() and c:IsType(TYPE_PENDULUM))
    card::is_type(d, c, TYPE_LINK) || (card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM))
}
/// `s.rescon(ft1,ft2,ft3,ft)`: the check function of the Extra Deck selection. `ft1` is only
/// captured by the Lua closure and never read there.
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>, ft2: i32, ft3: i32, ft: i32) -> (bool, bool) {
    // local exnpct=sg:FilterCount(s.exfilter1,nil)
    let exnpct = group::filter_count(d, sg, &exfilter1, NO_EX) as i32;
    // local expct=sg:FilterCount(s.exfilter2,nil)
    let expct = group::filter_count(d, sg, &exfilter2, NO_EX) as i32;
    // local groupcount=#sg
    let groupcount = sg.len() as i32;
    // local classcount=sg:GetClassCount(Card.GetCode)
    let classcount = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) as i32;
    // local res=ft2>=exnpct and ft3>=expct and ft>=groupcount and classcount==groupcount
    let res = ft2 >= exnpct && ft3 >= expct && ft >= groupcount && classcount == groupcount;
    // return res, not res
    (res, !res)
}
/// `s.tfop(e,tp,eg,ep,ev,re,r,rp)`
fn tfop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.tffilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| tffilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if let Some(tc) = tc {
        if duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_VIRTUAL_WORLD_GATE),tp,LOCATION_ONFIELD,0,nil)
            let ct = duel::get_matching_group_count(
                d,
                Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VIRTUAL_WORLD_GATE as u16])),
                a.tp,
                LOCATION_ONFIELD,
                0,
                NO_EX,
            );
            // if ct>=2 then
            if ct >= 2 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // local c=e:GetHandler()
                let c = effect::get_handler(d, a.e).nil();
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetTargetRange(LOCATION_MZONE,0)
                effect::set_target_range(d, e1, LOCATION_MZONE, 0);
                // e1:SetValue(200)
                effect::set_value(d, e1, 200);
                // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_VIRTUAL_WORLD))
                effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]))));
                // e1:SetReset(RESET_PHASE|PHASE_END)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, a.tp);
            }
            // if ct>=3 and Duel.IsPlayerCanDiscardDeck(tp,3) then
            if ct >= 3 && duel::is_player_can_discard_deck(d, a.tp, 3) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.DiscardDeck(tp,3,REASON_EFFECT)
                duel::discard_deck(d, a.tp, 3, REASON_EFFECT);
            }
            // if ct>=4 then
            if ct >= 4 {
                // local ft1=Duel.GetLocationCountFromEx(tp)
                let _ft1 = duel::get_location_count_from_ex(d, a.tp, None, NO_EX, duel::ExtraCard::None, None).0;
                // local ft2=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
                let mut ft2 = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Type(TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ), None).0;
                // local ft3=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_PENDULUM+TYPE_LINK)
                let mut ft3 = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Type(TYPE_PENDULUM + TYPE_LINK), None).0;
                // local ft=math.min(Duel.GetUsableMZoneCount(tp),4)
                let mut ft = duel::get_usable_m_zone_count(d, a.tp, None).min(4);
                // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then
                if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
                    // if ft1>0 then ft1=1 end
                    // (ft1 is passed to s.rescon, which never reads it: nothing to keep)
                    // if ft2>0 then ft2=1 end
                    if ft2 > 0 {
                        ft2 = 1;
                    }
                    // if ft3>0 then ft3=1 end
                    if ft3 > 0 {
                        ft3 = 1;
                    }
                    // ft=1
                    ft = 1;
                }
                // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
                let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, NO_EX);
                // if #sg>0 then
                if !sg.is_empty() {
                    // local rg=aux.SelectUnselectGroup(sg,e,tp,1,ft,s.rescon(ft1,ft2,ft3,ft),1,tp,HINTMSG_SPSUMMON)
                    let rescon_fn = |d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>| {
                        rescon(d, sg, e, tp, mg, c, ft2, ft3, ft)
                    };
                    let rg = aux::select_unselect_group(
                        d,
                        &sg,
                        Some(a.e),
                        a.tp,
                        Some(1),
                        Some(ft as usize),
                        Some(&rescon_fn),
                        1,
                        a.tp,
                        Some(HINTMSG_SPSUMMON as u64),
                        None,
                        None,
                        false,
                    )
                    .unwrap_or_default();
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SpecialSummon(rg,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, &rg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    }
}
