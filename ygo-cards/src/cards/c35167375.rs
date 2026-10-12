//! Surprise Fusion
//! サプライズ・フュージョン
use super::prelude::*;

pub const ID: u32 = 35167375;
/// `local TOKEN_SURPRISE=id+1`
const TOKEN_SURPRISE: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_SURPRISE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make 1 face-up monster you control become the declared Type and Attribute, then you can Fusion Summon 1 Fusion Monster from your Extra Deck, using monsters you control, including the targeted monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute 1 Fusion Monster, and if you do, Special Summon 2 "Surprise Tokens" (Spellcaster/DARK/ATK 0/DEF 0) with a Level equal to the Level the Tributed monster had on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_RELEASE | CATEGORY_SPECIAL_SUMMON | CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tokentg)
    effect::set_target(d, e2, Some(cost_target(tokentg)));
    // e2:SetOperation(s.tokenop)
    effect::set_operation(d, e2, Some(operation(tokenop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,chkc)` of e1 (`s.target`)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local race=Duel.AnnounceRace(tp,1,RACE_ALL)
    let race = duel::announce_race(d, tp, 1, RACE_ALL);
    // local attr=tc:IsRaceExcept(race) and Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL) or tc:AnnounceAnotherAttribute(tp)
    let attr = if card::is_race_except(d, tc, race, None, 0, None) {
        duel::announce_attribute(d, tp, 1, ATTRIBUTE_ALL) as u64
    } else {
        card::announce_another_attribute(d, tc, tp) as u64
    };
    // e:SetLabel(race,attr)
    effect::set_label(d, a.e, &[race as i64, attr as i64]);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local race,attr=e:GetLabel()
        let labels = effect::get_labels(d, e);
        // (nil where the target set fewer labels, e.g. an activation the belief replay forced:
        // Lua fails where it is used)
        let nil = |v: Option<&i64>| v.copied().unwrap_or_else(|| crate::lua_error("attempt to perform bitwise operation on a nil value"));
        let race = nil(labels.first()) as u64;
        // if tc:IsRaceExcept(race) then
        if card::is_race_except(d, tc, race, None, 0, None) {
            // --It becomes that Type
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            // e1:SetValue(race)
            effect::set_value(d, e1, race as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
        // if tc:IsAttributeExcept(attr) then
        let attr = nil(labels.get(1)) as u32;
        if card::is_attribute_except(d, tc, attr, None, 0, None) {
            // --It becomes that Attribute
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
            // e2:SetValue(attr)
            effect::set_value(d, e2, attr as i64);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
        // if not (tc:IsRace(race) or tc:IsAttribute(attr)) then return end
        if !(card::is_race(d, tc, race) || card::is_attribute(d, tc, attr)) {
            return;
        }
        // local fusion_params={matfilter=Fusion.OnFieldMat,gc=tc}
        let params = fusion::SummonEff {
            matfilter: Some(fusion::on_field_mat()),
            gc: Some(std::sync::Arc::new(move |_d, _a| Value::card(Some(tc)))),
            ..Default::default()
        };
        // if Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        let mut a0 = a.clone();
        a0.chk = Some(0);
        if fusion::summon_eff_tg_call(d, &params, &a0) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
            fusion::summon_eff_op_call(d, &params, a);
        }
    }
}
/// `s.tributefilter(c,tp)`
fn tributefilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFusionMonster() and c:IsReleasableByEffect() and Duel.GetMZoneCount(tp,c)>=2
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SURPRISE,0,TYPES_TOKEN,0,0,c:GetLevel(),RACE_SPELLCASTER,ATTRIBUTE_DARK)
    if !(card::is_fusion_monster(d, c) && card::is_releasable_by_effect(d, c, None, None)) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 < 2 {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_player_can_special_summon_monster(d, tp, TOKEN_SURPRISE, token_data(lv), None, None, 0)
}
/// `TYPES_TOKEN, 0, 0, level, RACE_SPELLCASTER, ATTRIBUTE_DARK` of `Duel.IsPlayerCanSpecialSummonMonster`
fn token_data(level: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(level),
        race: Some(RACE_SPELLCASTER),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `Duel.CheckReleaseGroup(tp,s.tributefilter,1,false,1,true,nil,tp,nil,false,nil,tp)` / the `SelectReleaseGroup` of tokenop
fn release_args(tp: u8) -> duel::ReleaseArgs {
    duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(1),
        cancelable: false,
        check_field: true,
        to_check: None,
        toplayer: Some(tp),
        zone: None,
        use_oppo: false,
    }
}
/// `s.tokentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.CheckReleaseGroup(tp,s.tributefilter,1,false,1,true,nil,tp,nil,false,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::check_release_group(d, tp, Some(&|d, c| tributefilter(d, c, tp)), 1, release_args(tp), NO_EX, false);
    }
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.tokenop(e,tp,eg,ep,ev,re,r,rp)`
fn tokenop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sc=Duel.SelectReleaseGroup(tp,s.tributefilter,1,1,false,false,true,nil,tp,nil,false,nil,tp):GetFirst()
    let sel = duel::select_release_group(d, tp, Some(&|d, c| tributefilter(d, c, tp)), 1, 1, release_args(tp), NO_EX, false).nil();
    let sc = match group::get_first(d, &sel) {
        Some(sc) => sc,
        // if not sc then return end
        None => return,
    };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc, true);
    // local lv=sc:GetLevel()
    let lv = card::get_level(d, sc);
    // if Duel.Release(sc,REASON_EFFECT)>0 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SURPRISE,0,TYPES_TOKEN,0,0,lv,RACE_SPELLCASTER,ATTRIBUTE_DARK) then
    if duel::release(d, sc, REASON_EFFECT, None) > 0
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_SURPRISE, token_data(lv), None, None, 0)
    {
        // for i=1,2 do
        for _ in 0..2 {
            // local token=Duel.CreateToken(tp,TOKEN_SURPRISE)
            let token = duel::create_token(d, tp, TOKEN_SURPRISE).nil();
            // token:Level(lv)
            card::info_level(d, token, Some(lv));
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
