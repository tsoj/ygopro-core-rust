//! Hecahands Jauzah
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 67021206;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HECAHANDS], material_setcode: &[SET_HECAHANDS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Hecahands" monster + 1 Illusion monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_HECAHANDS),aux.FilterBoolFunctionEx(Card.IsRace,RACE_ILLUSION))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_HECAHANDS as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_ILLUSION, Some(fc), st, Some(tp)))),
        ],
    );
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // Must first be either Fusion Summoned, or Special Summoned (from your Extra Deck) by Tributing 1 Illusion monster and 1 face-up monster you control owned by your opponent
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // e0:SetTarget(s.selfsptg)
    effect::set_target(d, e0, Some(lua_fn(selfsptg)));
    // e0:SetOperation(s.selfspop)
    effect::set_operation(d, e0, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Add 1 "Hecahands" card from your Deck or GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card battles a monster, neither can be destroyed by that battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.indestg)
    effect::set_target(d, e2, Some(effect_card_fn(indestg)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcostfilter(c,tp,fc)`
fn selfspcostfilter(d: &mut Duel, c: CardId, tp: u8, fc: Option<CardId>) -> bool {
    // return (c:IsRace(RACE_ILLUSION) or (c:IsFaceup() and c:IsOwner(1-tp))) and c:IsReleasable()
    //     and c:IsCanBeFusionMaterial(fc,MATERIAL_FUSION)
    (card::is_race(d, c, RACE_ILLUSION) || (card::is_faceup(d, c) && card::is_owner(d, c, 1 - tp)))
        && card::is_releasable(d, c, None, None)
        && card::is_can_be_fusion_material(d, c, fc, Some(MATERIAL_FUSION), None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetLocationCountFromEx(tp,tp,sg,e:GetHandler())>0
    //     and sg:IsExists(Card.IsRace,1,nil,RACE_ILLUSION) and sg:IsExists(aux.FaceupFilter(Card.IsOwner,1-tp),1,nil)
    let handler = effect::get_handler(d, e.nil()).nil();
    let cnt = duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(handler), None).0;
    let res = cnt > 0
        && group::is_exists(d, sg, &|d, c| card::is_race(d, c, RACE_ILLUSION), 1, NO_EX)
        && group::is_exists(d, sg, &aux::faceup_filter(|d, c| card::is_owner(d, c, 1 - tp)), 1, NO_EX);
    (res, false)
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if not c then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local mg=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_MZONE,0,nil,tp,c)
    let mg = duel::get_matching_group(d, Some(&|d, x| selfspcostfilter(d, x, tp, Some(c))), tp, LOCATION_MZONE, 0, NO_EX);
    // return #mg>=2 and aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon,0)
    mg.len() >= 2 && matches!(aux::select_unselect_group(d, &mg, Some(a.e(0)), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(9);
    // local mg=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_MZONE,0,nil,tp,c)
    let mg = duel::get_matching_group(d, Some(&|d, x| selfspcostfilter(d, x, tp, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let g = aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) -> Value {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if not g then return end
    if let Some(g) = g {
        // Duel.Release(g,REASON_COST|REASON_MATERIAL)
        duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
        // g:DeleteGroup()
    }
    Value::Nil
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_HECAHANDS) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_HECAHANDS as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(thfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.indestg(e,c)`
fn indestg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local handler=e:GetHandler()
    let handler = effect::get_handler(d, e).nil();
    // return c==handler or c==handler:GetBattleTarget()
    c == handler || card::get_battle_target(d, handler) == Some(c)
}
