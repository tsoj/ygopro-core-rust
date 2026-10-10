//! Alba System Dogmatikalamity
use super::prelude::*;

pub const ID: u32 = 93053159;

// s.listed_names={CARD_ALBAZ}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMixN(c,true,true,CARD_ALBAZ,1,s.matfilter,6)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::Code(CARD_ALBAZ), 1), (fusion::Mat::F(fusion::fus_mat_full(|d, x, a| matfilter(d, x, a))), 6)],
    );
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // This card can attack all monsters your opponent controls, once each
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e1, EFFECT_ATTACK_ALL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot declare an attack unless you send 1 card from your Extra Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_ATTACK_COST)
    effect::set_code(d, e2, EFFECT_ATTACK_COST);
    // e2:SetCost(function(e,c,tp) return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_EXTRA,0,1,nil) end)
    effect::set_cost(d, e2, Some(lua_fn(attack_cost)));
    // e2:SetOperation(s.atkcostop)
    effect::set_operation(d, e2, Some(operation(atkcostop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send all cards in both players' Extra Decks to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.tgcon)
    effect::set_condition(d, e3, Some(condition(tgcon)));
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,fc,sumtype,tp,sub,mg,sg)`
fn matfilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsControler(tp)
    //   and (not sg or not sg:IsExists(s.fusfilter,1,c,c:GetCode(fc,sumtype,tp),fc,sumtype,tp))
    if !(card::is_location(d, c, LOCATION_GRAVE) && card::is_controler(d, c, a.tp)) {
        return false;
    }
    match a.sg {
        None => true,
        Some(sg) => {
            // (c:GetCode(fc,sumtype,tp) is not the last argument: only its first code is passed)
            let code = card::get_code_ctx(d, c, Some(a.fc), a.sumtype, Some(a.tp)).first().copied().unwrap_or(0);
            let (fc, sumtype, tp) = (a.fc, a.sumtype, a.tp);
            !group::is_exists(d, sg, &|d, x| fusfilter(d, x, code, fc, sumtype, tp), 1, c.into())
        }
    }
}
/// `s.fusfilter(c,code,fc,sumtype,tp)`
fn fusfilter(d: &mut Duel, c: CardId, code: u32, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSummonCode(fc,sumtype,tp,code) and not c:IsHasEffect(511002961)
    card::is_summon_code(d, c, Some(fc), sumtype, tp, &[code]) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// `function(e,c,tp) return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_EXTRA,0,1,nil) end`
fn attack_cost(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.atkcostop(e,tp,...)`
fn atkcostop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsAttackCostPaid()~=2 and e:GetHandler():IsLocation(LOCATION_MZONE) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, h, LOCATION_MZONE) {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_EXTRA,0,nil)
        let g = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| card::is_able_to_grave_as_cost(d, c)), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // local sg=aux.SelectUnselectGroup(g,e,tp,0,1,nil,1,tp,HINTMSG_TOGRAVE,function() return Duel.IsAttackCostPaid()==0 end,nil)
        let finishcon = |d: &mut Duel, _sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| (duel::is_attack_cost_paid(d) == 0, false);
        let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(0), Some(1), None, 1, a.tp, Some(HINTMSG_TOGRAVE as u64), Some(&finishcon), None, false)
            .unwrap_or_default();
        // if #sg==1 then
        if sg.len() == 1 {
            // Duel.SendtoGrave(sg,REASON_COST)
            duel::send_to_grave(d, sg, REASON_COST, None, None);
            // Duel.AttackCostPaid()
            duel::attack_cost_paid(d, None);
        } else {
            // Duel.AttackCostPaid(2)
            duel::attack_cost_paid(d, Some(2));
        }
    }
}
/// `s.tgconfilter(c)`
fn tgconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FUSION) and c:ListsCodeAsMaterial(CARD_ALBAZ)
    card::is_type(d, c, TYPE_FUSION) && card::lists_code_as_material(d, c, &[CARD_ALBAZ])
}
/// `s.tgcon(e,tp,...)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not e:GetHandler():IsFusionSummoned() then return false end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_fusion_summoned(d, h) {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tgconfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&tgconfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)>=6
    group::get_class_count(d, &g, &|d, cc| card::get_code(d, cc).first().copied().unwrap_or(0) as i64) >= 6
}
/// `s.tgtg(e,tp,...,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_EXTRA,LOCATION_EXTRA)
    let g = duel::get_field_group(d, a.tp, LOCATION_EXTRA, LOCATION_EXTRA);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, a.tp, 0);
    true
}
/// `s.tgop(e,tp,...)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_EXTRA,LOCATION_EXTRA)
    let g = duel::get_field_group(d, a.tp, LOCATION_EXTRA, LOCATION_EXTRA);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
