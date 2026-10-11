//! Clock Lizard
//! クロック・リザード
use super::prelude::*;

pub const ID: u32 = 51476410;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_CYBERSE),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))), 2, None, None, None, None);
    // Fusion Summon
    // local e1=Fusion.CreateSummonEff({handler=c,location=LOCATION_GRAVE,fusfilter=s.lizardcheck,matfilter=aux.FALSE,
    //   extrafil=s.extrafil,preselect=s.preop,desc=aux.Stringid(id,0),extraop=Fusion.BanishMaterial,nosummoncheck=true,chkf=PLAYER_NONE})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            location: Some(LOCATION_GRAVE),
            fusfilter: Some(fusion::fus_filter(|d, c, tp| lizardcheck(d, c, tp))),
            matfilter: Some(std::sync::Arc::new(|_d, _c, _e, _tp, _chk| false)),
            extrafil: Some(std::sync::Arc::new(extrafil)),
            preselect: Some(std::sync::Arc::new(|d, _e, tc| duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0)),
            desc: Some(aux::stringid(ID, 0)),
            extraop: Some(fusion::banish_material()),
            nosummoncheck: true,
            chkf: Some(PLAYER_NONE as u64),
            ..Default::default()
        },
    );
    // e1:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget((function(tg) return function(e,tp,eg,ep,ev,re,r,rp,chk) ... end end)(e1:GetTarget()))
    let tg = effect::get_target(d, e1);
    effect::set_target(d, e1, Some(lua_fn(move |d, a: Args| -> Value {
        let tp = a.p(1);
        if a.v(8).int() == 0 {
            // if chk==0 then return Duel.IsPlayerCanRemove(tp) and aux.CheckSummonGate(tp,2) and tg(e,tp,eg,ep,ev,re,r,rp,chk) end
            if !duel::is_player_can_remove(d, tp, None) || !aux::check_summon_gate(d, tp, 2) {
                return Value::Bool(false);
            }
            let Some(tg) = &tg else { crate::lua_error("attempt to call a nil value") };
            return tg.call(d, a.0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, tp, LOCATION_GRAVE as i32);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_GRAVE as i32);
        Value::Nil
    })));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Decrease ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lizardcheck(c,tp)`
fn lizardcheck(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToExtra() and not c:IsHasEffect(CARD_CLOCK_LIZARD)
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,c:GetOriginalCode(),{c:GetOriginalSetCard()},c:GetOriginalType(),
    //   c:GetBaseAttack(),c:GetBaseDefense(),c:GetOriginalLevel(),c:GetOriginalRace(),c:GetOriginalAttribute())
    if !card::is_able_to_extra(d, c) || !card::is_has_effect(d, c, CARD_CLOCK_LIZARD, None).is_empty() {
        return false;
    }
    let md = duel::MonsterData {
        setcodes: Some(card::get_original_set_card(d, c)),
        type_: Some(card::get_original_type(d, c)),
        attack: Some(card::get_base_attack(d, c)),
        defense: Some(card::get_base_defense(d, c)),
        level: Some(card::get_original_level(d, c)),
        race: Some(card::get_original_race(d, c)),
        attribute: Some(card::get_original_attribute(d, c)),
    };
    let code = card::get_original_code(d, c);
    duel::is_player_can_special_summon_monster(d, tp, code, md, None, None, 0)
}
/// `s.checkextra(c)`: the function `function(tp,sg,fc)` for the Extra Deck Fusion check
fn checkextra(h: CardId) -> LuaFn {
    lua_fn(move |d, a: Args| -> bool {
        let tp = a.p(0);
        let sg = a.g(1);
        let fc = a.c(2);
        // return (fc:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,sg,fc) or Duel.GetLocationCountFromEx(tp,tp,sg+c,TYPE_FUSION))>0
        let count = if card::is_location(d, fc, LOCATION_EXTRA) {
            duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), duel::ExtraCard::Card(fc), None).0
        } else {
            let mut sgc = sg.clone();
            group::add_card(&mut sgc, &CardSet::from_card(h));
            duel::get_location_count_from_ex(d, tp, Some(tp), (&sgc).into(), duel::ExtraCard::Type(TYPE_FUSION), None).0
        };
        count > 0
    })
}
/// `s.extrafil(e,tp,mg)`
fn extrafil(d: &mut Duel, e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,
    //   Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) and LOCATION_MZONE or LOCATION_GRAVE,0,nil),
    //   s.checkextra(e:GetHandler())
    let loc = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() { LOCATION_MZONE } else { LOCATION_GRAVE };
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, loc, 0, NO_EX);
    let h = effect::get_handler(d, e).nil();
    (Some(g), Some(checkextra(h)))
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsPreviousLocation(LOCATION_GRAVE)
    card::is_faceup(d, c) && card::is_previous_location(d, c, LOCATION_GRAVE)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSpecialSummoned()
    card::is_faceup(d, c) && card::is_special_summoned(d, c)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_GRAVE,0,1,nil,RACE_CYBERSE)
    //   and Duel.IsExistingMatchingCard(s.atkfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_CYBERSE)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&atkfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.atkfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&atkfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local atk=Duel.GetMatchingGroupCount(Card.IsRace,tp,LOCATION_GRAVE,0,nil,RACE_CYBERSE)*400
    let atk = duel::get_matching_group_count(d, Some(&|d, c| card::is_race(d, c, RACE_CYBERSE)), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 400;
    // if atk==0 then return end
    if atk == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-atk)
        effect::set_value(d, e1, -atk);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
